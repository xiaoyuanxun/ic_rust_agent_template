mod did;
mod utils;
use candid::{Principal, Encode, Decode};

// use crate::did::{};
use crate::utils::build_agent;

pub const USERA: &str = "identity/1.pem";
pub const USERB: &str = "identity/2.pem";
pub const USERC: &str = "identity/3.pem";
pub const USERD: &str = "identity/4.pem";
pub const USERE: &str = "identity/5.pem";

// pub async fn call_canister() {
//     let canister = Principal::from_text("").unwrap();
    
//     let response_blob = build_agent("./identity.pem")
//         .update(
//             &canister,
//             "method_name"
//         )
//         .with_arg(Encode!().unwrap())
//         .call_and_wait()
//         .await
//         .expect("Call Response Error !");
//     let result = Decode!(&response_blob, CallResultType).unwrap();

//     println!("result : {:?}", result);
// }

#[tokio::main]
async fn main() {

}
