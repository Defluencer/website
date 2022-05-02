#![recursion_limit = "1024"]

mod app;
mod components;
mod pages;
mod utils;

use crate::{
    app::Props,
    utils::{IpfsService, LocalStorage, Web3Service},
};

/// ENS Domain name with "defluencer" as subdomain. egg. defluencer.sionois.eth
/// OR a beacon CID.
const BEACON: &str = "bafyreihvbmv7rgtqa5zlg4v2l7elbo5hwshrhzg5dfjphe54qdg2ay5s2i";
//const BEACON: &str = "sionois";

fn main() {
    let web3 = Web3Service::new();
    let storage = LocalStorage::new();
    let ipfs = IpfsService::new(&storage);

    let props = Props {
        web3,
        ipfs,
        storage,
        beacon: BEACON,
    };

    yew::start_app_with_props::<app::App>(props);
}
