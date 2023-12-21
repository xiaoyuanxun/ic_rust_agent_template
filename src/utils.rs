use ic_agent::{identity, agent::http_transport};

const ICP_LEDGER_CANISTER_TEXT: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";

const GHOST_LEDGER_CANISTER_TEXT: &str = "4c4fd-caaaa-aaaaq-aaa3a-cai";

const CKBTC_LEDGER_CANISTER_TEXT: &str = "mxzaz-hqaaa-aaaar-qaada-cai";

const CKETH_LEDGER_CANISTER_TEXT: &str = "ss2fx-dyaaa-aaaar-qacoq-cai";



pub fn build_agent(pem_identity_path: &str) -> ic_agent::Agent {
    let url = "https://ic0.app".to_string();
    let identity = identity::Secp256k1Identity::from_pem_file(String::from(pem_identity_path)).expect("not found identity pem");
    let transport = http_transport::ReqwestTransport::create(&url).expect("create transport error");
    let agent = ic_agent::Agent::builder()
        .with_url(url)
        .with_transport(transport)
        .with_identity(identity)
        .build()
        .expect("build agent error");
    agent
}
