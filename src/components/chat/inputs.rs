use std::{rc::Rc, str};

use crate::utils::{IpfsService, LocalStorage, Web3Service};

use wasm_bindgen_futures::spawn_local;

use yew::{
    prelude::{classes, html, Component, ComponentLink, Html, Properties, ShouldRender},
    services::ConsoleService,
};

use cid::Cid;

use linked_data::{
    chat::{ChatId, ChatMessage, ChatSig, MessageType},
    live::Live,
    signature::SignedMessage,
    PeerId,
};

use web3::types::Address;

const SIGN_MSG_KEY: &str = "signed_message";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

enum DisplayState {
    /// Before anything, ask user to connect account.
    Connect,
    /// Has use reverse resolution to get a name from an address.
    NameResolved,
    /// The signature is ready and can be use for messages.
    Chatting,
}

pub struct Inputs {
    props: Props,
    link: ComponentLink<Self>,

    state: DisplayState,

    temp_msg: String,

    address: Option<Address>,
    peer_id: Option<PeerId>,
    name: Option<String>,
    sign_msg_content: Option<ChatId>,
    sign_msg_cid: Option<ChatSig>,
}

pub enum Msg {
    Set(String),
    Enter,
    Connect,
    PeerID(Result<PeerId>),
    Account(Result<Address>),
    AccountName(Result<String>),
    SetName(String),
    SubmitName,
    Signed(Result<[u8; 65]>),
    Minted(Result<Cid>),
    Recover(Result<SignedMessage<ChatId>>),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub ipfs: IpfsService,
    pub web3: Web3Service,
    pub storage: LocalStorage,
    pub live: Rc<Live>,
}

impl Component for Inputs {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let sign_msg_cid = props.storage.get_cid(SIGN_MSG_KEY);

        if let Some(cid) = sign_msg_cid {
            spawn_local({
                let cb = link.callback_once(Msg::Recover);
                let ipfs = props.ipfs.clone();

                async move { cb.emit(ipfs.dag_get(cid, Option::<String>::None).await) }
            });
        }

        #[cfg(debug_assertions)]
        ConsoleService::info("Chat Inputs Created");

        Self {
            props,
            link,

            state: DisplayState::Connect,

            temp_msg: String::default(),

            address: None,
            peer_id: None,
            name: None,
            sign_msg_content: None,
            sign_msg_cid,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Set(msg) => self.on_chat_input(msg),
            Msg::Enter => self.send_message(),
            Msg::Connect => self.connect_account(),
            Msg::PeerID(res) => self.on_peer_id(res),
            Msg::Account(res) => self.on_account_connected(res),
            Msg::AccountName(res) => self.on_account_name(res),
            Msg::SetName(name) => self.on_name_input(name),
            Msg::SubmitName => self.on_name_submit(),
            Msg::Signed(res) => self.on_signature(res),
            Msg::Minted(res) => self.on_signed_msg_added(res),
            Msg::Recover(res) => self.on_signed_msg_recovered(res),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if !Rc::ptr_eq(&self.props.live, &props.live) {
            self.props = props;

            #[cfg(debug_assertions)]
            ConsoleService::info("Chat Inputs Page Changed");

            return true;
        }

        false
    }

    fn view(&self) -> Html {
        html! {
            <ybc::Box>
            {
                match &self.state {
                    DisplayState::Connect => self.connect_dialog(),
                    DisplayState::NameResolved => self.name_dialog(),
                    DisplayState::Chatting => self.chat_dialog(),
                }
            }
            </ybc::Box>
        }
    }
}

impl Inputs {
    fn connect_dialog(&self) -> Html {
        html! {
            <ybc::Field label="To chat, please connect Metamask".to_owned() >
                <ybc::Button classes=classes!("is-primary") onclick=self.link.callback(|_| Msg::Connect) >
                    { "Connect" }
                </ybc::Button>
            </ybc::Field>
        }
    }

    fn name_dialog(&self) -> Html {
        let name = match &self.name {
            Some(name) => name.clone(),
            None => String::new(),
        };

        html! {
            <>
                <ybc::Field label="Display Name".to_owned() >
                    <ybc::Control>
                        <ybc::Input name="chat_name" value=name update=self.link.callback(Msg::SetName) />
                    </ybc::Control>
                </ybc::Field>
                <ybc::Field label="Confirm your name by signing it".to_owned() >
                    <ybc::Control>
                        <ybc::Button classes=classes!("is-primary") onclick=self.link.callback(|_| Msg::SubmitName)>
                            { "Sign" }
                        </ybc::Button>
                    </ybc::Control>
                </ybc::Field>
            </>
        }
    }

    fn chat_dialog(&self) -> Html {
        html! {
            <>
                <ybc::Field>
                    <ybc::Control>
                        <ybc::TextArea name="chat_msg" value=String::default() update=self.link.callback(Msg::Set) rows=3 fixed_size=true />
                    </ybc::Control>
                </ybc::Field>
                <ybc::Field>
                    <ybc::Control>
                        <ybc::Button classes=classes!("is-primary") onclick=self.link.callback(|_| Msg::Enter)>
                            { "Send" }
                        </ybc::Button>
                    </ybc::Control>
                </ybc::Field>
            </>
        }
    }

    /// Send chat message via gossipsub.
    fn send_message(&mut self) -> bool {
        let text = self.temp_msg.clone();

        let cid = match self.sign_msg_cid {
            Some(cid) => cid,
            None => {
                #[cfg(debug_assertions)]
                ConsoleService::error("No Signed Message CID");
                return false;
            }
        };

        let chat_msg = ChatMessage {
            message: MessageType::Text(text),
            signature: cid.into(),
        };

        let json_string = match serde_json::to_string(&chat_msg) {
            Ok(json_string) => json_string,
            Err(e) => {
                ConsoleService::error(&format!("{:#?}", e));
                return false;
            }
        };

        #[cfg(debug_assertions)]
        ConsoleService::info("Publish Message");

        spawn_local({
            let ipfs = self.props.ipfs.clone();
            let topic = self.props.live.chat_topic.clone();

            async move {
                if let Err(e) = ipfs.pubsub_pub(topic, json_string).await {
                    ConsoleService::error(&format!("{:#?}", e));
                }
            }
        });

        true
    }

    /// Trigger ethereum request accounts.
    fn connect_account(&self) -> bool {
        #[cfg(debug_assertions)]
        ConsoleService::info("Get Address");

        spawn_local({
            let cb = self.link.callback(Msg::Account);
            let web3 = self.props.web3.clone();

            async move { cb.emit(web3.get_eth_accounts().await) }
        });

        #[cfg(debug_assertions)]
        ConsoleService::info("Get Peer ID");

        spawn_local({
            let cb = self.link.callback(Msg::PeerID);
            let ipfs = self.props.ipfs.clone();

            async move { cb.emit(ipfs.ipfs_node_id().await) }
        });

        false
    }

    fn on_chat_input(&mut self, msg: String) -> bool {
        if msg.ends_with('\n') {
            self.temp_msg = msg;

            return self.send_message();
        }

        self.temp_msg = msg;

        false
    }

    fn on_name_input(&mut self, name: String) -> bool {
        self.name = Some(name);

        false
    }

    fn on_peer_id(&mut self, response: Result<PeerId>) -> bool {
        let peer_id = match response {
            Ok(id) => id,
            Err(e) => {
                ConsoleService::error(&format!("{:?}", e));
                self.state = DisplayState::Connect;
                return true;
            }
        };

        #[cfg(debug_assertions)]
        ConsoleService::info(&format!("Peer ID => {:?}", &peer_id));

        self.peer_id = Some(peer_id);

        false
    }

    /// Callback when Metamask get_eth_accounts return an address.
    fn on_account_connected(&mut self, response: Result<Address>) -> bool {
        let address = match response {
            Ok(address) => address,
            Err(e) => {
                ConsoleService::error(&format!("{:?}", e));
                self.state = DisplayState::Connect;
                return true;
            }
        };

        #[cfg(debug_assertions)]
        ConsoleService::info(&format!("Address => {}", &address.to_string()));

        self.address = Some(address);

        spawn_local({
            let cb = self.link.callback(Msg::AccountName);
            let web3 = self.props.web3.clone();

            async move { cb.emit(web3.reverse_resolve(address).await) }
        });

        false
    }

    /// Callback when web3 reverse resolve an address and returns a name.
    fn on_account_name(&mut self, response: Result<String>) -> bool {
        let name = match response {
            Ok(string) => string,
            Err(e) => {
                ConsoleService::error(&format!("{:?}", e));

                String::new()
            }
        };

        #[cfg(debug_assertions)]
        ConsoleService::info(&format!("Address Reverse Revolve => {}", &name));

        self.name = Some(name);
        self.state = DisplayState::NameResolved;

        true
    }

    /// Callback when the chat name choice is submited.
    fn on_name_submit(&mut self) -> bool {
        let name = match self.name.take() {
            Some(name) => name,
            None => {
                #[cfg(debug_assertions)]
                ConsoleService::error("No Name");
                return false;
            }
        };

        #[cfg(debug_assertions)]
        ConsoleService::info("Name Submitted");

        let address = match self.address {
            Some(addrs) => addrs,
            None => {
                #[cfg(debug_assertions)]
                ConsoleService::error("No Address");
                return false;
            }
        };

        let peer_id = match self.peer_id.take() {
            Some(peer) => peer,
            None => {
                #[cfg(debug_assertions)]
                ConsoleService::error("No Peer Id");
                return false;
            }
        };

        let data = ChatId { peer_id, name };

        spawn_local({
            let cb = self.link.callback_once(Msg::Signed);
            let web3 = self.props.web3.clone();
            let data = data.clone();

            async move { cb.emit(web3.eth_sign(address, data).await) }
        });

        self.sign_msg_content = Some(data);

        false
    }

    /// Callback when the chat ID has been signed.
    fn on_signature(&mut self, response: Result<[u8; 65]>) -> bool {
        let signature = match response {
            Ok(sig) => sig,
            Err(e) => {
                ConsoleService::error(&format!("{:?}", e));
                self.state = DisplayState::Connect;
                return true;
            }
        };

        #[cfg(debug_assertions)]
        ConsoleService::info("Signature Received");

        let address = match self.address.take() {
            Some(addrs) => addrs.to_fixed_bytes(),
            None => {
                #[cfg(debug_assertions)]
                ConsoleService::error("No Address");
                return false;
            }
        };

        let data = match self.sign_msg_content.take() {
            Some(data) => data,
            None => {
                #[cfg(debug_assertions)]
                ConsoleService::error("No Signed Message Content");
                return false;
            }
        };

        let signed_msg = SignedMessage {
            address,
            data,
            signature,
        };

        #[cfg(debug_assertions)]
        ConsoleService::info(&format!("Verifiable => {}", &signed_msg.verify()));

        spawn_local({
            let cb = self.link.callback_once(Msg::Minted);
            let ipfs = self.props.ipfs.clone();

            async move { cb.emit(ipfs.dag_put(&signed_msg).await) }
        });

        false
    }

    ///Callback when the signed chat ID has been added to IPFS.
    fn on_signed_msg_added(&mut self, response: Result<Cid>) -> bool {
        let cid = match response {
            Ok(cid) => cid,
            Err(e) => {
                ConsoleService::error(&format!("{:?}", e));
                self.state = DisplayState::Connect;
                return true;
            }
        };

        #[cfg(debug_assertions)]
        ConsoleService::info("Signed Message Added");

        self.props.storage.set_cid(SIGN_MSG_KEY, &cid);

        self.sign_msg_cid = Some(cid);
        self.state = DisplayState::Chatting;

        true
    }

    /// Callback when a signed message is recovered from a previous session.
    fn on_signed_msg_recovered(&mut self, response: Result<SignedMessage<ChatId>>) -> bool {
        let signed_msg = match response {
            Ok(msg) => msg,
            Err(e) => {
                #[cfg(debug_assertions)]
                ConsoleService::error(&format!("{:?}", e));

                self.props.storage.remove_item(SIGN_MSG_KEY);

                return false;
            }
        };

        if !signed_msg.verify() {
            self.props.storage.remove_item(SIGN_MSG_KEY);

            return false;
        }

        #[cfg(debug_assertions)]
        ConsoleService::info("Signed Message Recovered");

        self.state = DisplayState::Chatting;

        true
    }
}
