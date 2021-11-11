use core::fmt;

use std::{borrow::Cow, convert::TryFrom, rc::Rc};

use crate::utils::local_storage::LocalStorage;

use futures_util::{
    future::{AbortRegistration, Abortable},
    join, AsyncBufReadExt, TryStreamExt,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use yew::{services::ConsoleService, Callback};

use cid::{multibase::Base, multihash::MultihashGeneric, Cid};

use linked_data::{peer_id_from_str, PeerId};

use reqwest::{multipart::Form, Client, Url};

pub const DEFAULT_URI: &str = "http://127.0.0.1:5001/api/v0/";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone)]
pub struct IpfsService {
    client: Client,
    base_url: Rc<Url>,
}

impl IpfsService {
    pub fn new(storage: &LocalStorage) -> Self {
        let result = match storage.get_local_ipfs_addrs() {
            Some(addrs) => Url::parse(&addrs),
            None => {
                storage.set_local_ipfs_addrs(DEFAULT_URI);

                Url::parse(DEFAULT_URI)
            }
        };

        let url = match result {
            Ok(url) => url,
            Err(e) => {
                ConsoleService::error(&format!("{:#?}", e));
                std::process::abort();
            }
        };

        let client = Client::new();
        let base_url = Rc::from(url);

        Self { client, base_url }
    }

    /// Download content from block with this CID.
    pub async fn cid_cat(&self, cid: Cid) -> Result<Vec<u8>> {
        let url = self.base_url.join("cat")?;

        let bytes = self
            .client
            .post(url)
            .query(&[("arg", &cid.to_string())])
            .send()
            .await?
            .bytes()
            .await?;

        Ok(bytes.to_vec())
    }

    /// Download content simultaneously from 2 paths.
    pub async fn double_path_cat<U>(
        &self,
        audio_path: U,
        video_path: U,
    ) -> Result<(Vec<u8>, Vec<u8>)>
    where
        U: Into<Cow<'static, str>>,
    {
        let url = self.base_url.join("cat")?;

        let (audio_res, video_res) = join!(
            self.client
                .post(url.clone())
                .query(&[("arg", &audio_path.into())])
                .send(),
            self.client
                .post(url)
                .query(&[("arg", &video_path.into())])
                .send()
        );

        let audio_data = audio_res?;
        let video_data = video_res?;

        let (audio_result, video_result) = join!(audio_data.bytes(), video_data.bytes(),);

        let audio_data = audio_result?;
        let video_data = video_result?;

        Ok((audio_data.to_vec(), video_data.to_vec()))
    }

    /// Serialize then add dag node to IPFS. Return a CID.
    pub async fn dag_put<T>(&self, node: &T) -> Result<Cid>
    where
        T: ?Sized + Serialize,
    {
        #[cfg(debug_assertions)]
        ConsoleService::info(&format!(
            "Serde: Serialize => {}",
            serde_json::to_string(node).unwrap()
        ));

        let data = serde_json::to_string(node)?;

        //Reqwest was hacked to properly format multipart request with text ONLY
        let form = Form::new().text("object data", data);

        let url = self.base_url.join("dag/put")?;

        let res = self.client.post(url).multipart(form).send().await?;

        let res = match res.json::<DagPutResponse>().await {
            Ok(res) => res,
            Err(e) => return Err(e.into()),
        };

        let cid = Cid::try_from(res.cid.cid_string)?;

        #[cfg(debug_assertions)]
        ConsoleService::info(&format!("IPFS: dag put => {}", cid));

        Ok(cid)
    }

    /// Deserialize dag node from IPFS path. Return dag node.
    pub async fn dag_get<U, T>(&self, cid: Cid, path: Option<U>) -> Result<T>
    where
        U: Into<Cow<'static, str>>,
        T: ?Sized + DeserializeOwned,
    {
        let mut origin = cid.to_string();

        if let Some(path) = path {
            origin.push_str(&path.into());
        }

        #[cfg(debug_assertions)]
        ConsoleService::info(&format!("IPFS: dag get => {}", origin));

        let url = self.base_url.join("dag/get")?;

        let res = self
            .client
            .post(url)
            .query(&[("arg", &origin)])
            .send()
            .await?;

        let node = res.json::<T>().await?;

        Ok(node)
    }

    /// Resolve IPNS link then dag get. Return IPNS link, CID & Node.
    pub async fn resolve_and_dag_get<T>(&self, ipns: Cid) -> Result<(Cid, T)>
    where
        T: ?Sized + DeserializeOwned,
    {
        let url = self.base_url.join("name/resolve")?;

        let res = self
            .client
            .post(url)
            .query(&[("arg", &ipns.to_string())])
            .send()
            .await?;

        let res = match res.json::<NameResolveResponse>().await {
            Ok(res) => res,
            Err(e) => return Err(e.into()),
        };

        let cid = Cid::try_from(res.path)?;

        #[cfg(debug_assertions)]
        ConsoleService::info(&format!("IPFS: name resolve {} \n to {}", ipns, cid));

        let node = self.dag_get(cid, Option::<&str>::None).await?;

        Ok((cid, node))
    }

    pub async fn ipfs_node_id(&self) -> Result<PeerId> {
        let url = self.base_url.join("id")?;

        let res = self.client.post(url).send().await?;

        let res = match res.json::<IdResponse>().await {
            Ok(res) => res,
            Err(e) => return Err(e.into()),
        };

        let peer_id = peer_id_from_str(&res.id)?;

        Ok(peer_id)
    }

    pub async fn pubsub_pub<U>(&self, topic: U, msg: U) -> Result<()>
    where
        U: Into<Cow<'static, str>>,
    {
        let url = self.base_url.join("pubsub/pub")?;

        self.client
            .post(url)
            .query(&[("arg", &topic.into()), ("arg", &msg.into())])
            .send()
            .await?;

        Ok(())
    }

    /// Subscribe to a topic then deserialize output.
    pub async fn pubsub_sub<U>(
        &self,
        topic: U,
        cb: Callback<Result<(PeerId, Vec<u8>)>>,
        regis: AbortRegistration,
    ) where
        U: Into<Cow<'static, str>>,
    {
        if let Err(e) = self.pubsub_stream(&topic.into(), cb.clone(), regis).await {
            cb.emit(Err(e));
        }

        #[cfg(debug_assertions)]
        ConsoleService::info("Stream Dropped");
    }

    async fn pubsub_stream(
        &self,
        topic: &str,
        cb: Callback<Result<(PeerId, Vec<u8>)>>,
        regis: AbortRegistration,
    ) -> Result<()> {
        let url = self.base_url.join("pubsub/sub")?;

        let response = self
            .client
            .post(url)
            .query(&[("arg", topic)])
            .send()
            .await?;

        let stream = response.bytes_stream();

        let line_stream = stream.err_into().into_async_read().lines();

        let mut abortable_stream = Abortable::new(line_stream, regis);

        while let Some(line) = abortable_stream.try_next().await? {
            if let Ok(response) = serde_json::from_str::<PubsubSubResponse>(&line) {
                let PubsubSubResponse { from, data } = response;

                let from = Base::Base64Pad.decode(from)?;
                let data = Base::Base64Pad.decode(data)?;

                //Use Peer ID as CID v1 instead of multihash btc58 encoded
                // https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md#string-representation
                let multihash = MultihashGeneric::from_bytes(&from)?;
                let cid = Cid::new_v1(0x70, multihash);

                cb.emit(Ok((cid, data)));

                continue;
            }

            let ipfs_error = serde_json::from_str::<IPFSError>(&line)?;

            cb.emit(Err(ipfs_error.into()));
        }

        Ok(())
    }
}

#[derive(Deserialize)]
struct PubsubSubResponse {
    pub from: String,
    pub data: String,
}

#[derive(Deserialize)]
struct DagPutResponse {
    #[serde(rename = "Cid")]
    pub cid: CidString,
}

#[derive(Deserialize)]
struct CidString {
    #[serde(rename = "/")]
    pub cid_string: String,
}

#[derive(Deserialize)]
struct NameResolveResponse {
    #[serde(rename = "Path")]
    pub path: String,
}

#[derive(Deserialize)]
struct IdResponse {
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IPFSError {
    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "Code")]
    pub code: u64,

    #[serde(rename = "Type")]
    pub error_type: String,
}

impl std::error::Error for IPFSError {}

impl fmt::Display for IPFSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match serde_json::to_string_pretty(&self) {
            Ok(e) => write!(f, "{}", e),
            Err(e) => write!(f, "{}", e),
        }
    }
}
