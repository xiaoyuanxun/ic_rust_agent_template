use candid::{CandidType, Deserialize, Principal, Encode, Decode, Nat};
use crate::utils;

#[derive(CandidType, Deserialize, Debug)]
pub struct Account {
  pub owner: Principal,
  pub subaccount: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct ICRCTransferArg {
  pub to: Account,
  pub fee: Option<candid::Nat>,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub from_subaccount: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct ApproveArgs {
  pub fee: Option<candid::Nat>,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub from_subaccount: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub expected_allowance: Option<candid::Nat>,
  pub expires_at: Option<u64>,
  pub spender: Account,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ApproveError {
  GenericError{ message: String, error_code: candid::Nat },
  TemporarilyUnavailable,
  Duplicate{ duplicate_of: candid::Nat },
  BadFee{ expected_fee: candid::Nat },
  AllowanceChanged{ current_allowance: candid::Nat },
  CreatedInFuture{ ledger_time: u64 },
  TooOld,
  Expired{ ledger_time: u64 },
  InsufficientFunds{ balance: candid::Nat },
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ApproveResult { Ok(candid::Nat), Err(ApproveError) }

#[derive(CandidType, Deserialize, Debug)]
pub struct AllowanceArgs {
    pub account: Account, 
    pub spender: Account 
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Allowance { pub allowance: candid::Nat, pub expires_at: Option<u64> }

#[derive(CandidType, Deserialize, Debug)]
pub struct TransferFromArgs {
  pub to: Account,
  pub fee: Option<candid::Nat>,
  pub spender_subaccount: Option<serde_bytes::ByteBuf>,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum TransferError {
  GenericError{ message: String, error_code: candid::Nat },
  TemporarilyUnavailable,
  BadBurn{ min_burn_amount: candid::Nat },
  Duplicate{ duplicate_of: candid::Nat },
  BadFee{ expected_fee: candid::Nat },
  CreatedInFuture{ ledger_time: u64 },
  TooOld,
  InsufficientFunds{ balance: candid::Nat },
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ICRCTransferResult { 
  Ok(candid::Nat), 
  Err(TransferError) 
}

pub async fn icrc1_transfer(
  icrc_canister: Principal,
  from_agent: ic_agent::Agent,
  to: Account,
  amount: usize,
  memo: Option<serde_bytes::ByteBuf>,
) -> Result<Nat, TransferError> {
  let response_blob = from_agent
    .update(
      &icrc_canister,
      "icrc1_transfer"
    )
    .with_arg(Encode!(&ICRCTransferArg {
      to: to,
      fee: None,
      memo: memo,
      from_subaccount: None,
      created_at_time: None,
      amount: Nat::from(amount)
    }).unwrap())
    .call_and_wait()
    .await
    .expect("icrc1_transfer Response Error !");
  match Decode!(&response_blob, ICRCTransferResult).unwrap() {
    ICRCTransferResult::Ok(block_index) => Ok(block_index),
    ICRCTransferResult::Err(err) => Err(err)
  }
}


pub async fn icrc2_approve(
  from_agent: ic_agent::Agent,
  icrc_canister: Principal,
  args: ApproveArgs
) -> Result<u128, ApproveError> {
  let response_blob = from_agent
    .update(
      &icrc_canister, 
      "icrc2_approve"
    )
    .with_arg(Encode!(&args).unwrap())
    .call_and_wait()
    .await.unwrap();
  match Decode!(&response_blob, ApproveResult).unwrap() {
      ApproveResult::Ok(tx_index) => Ok(utils::nat_to_u128(tx_index)),
      ApproveResult::Err(err) => Err(err)
  }
}

pub async fn icrc2_allowance(
  from_agent: ic_agent::Agent,
  icrc_canister: Principal,
  args: AllowanceArgs
) -> Allowance {
  let response_blob = from_agent
    .query(
      &icrc_canister, 
      "icrc2_allowance"
    )
    .with_arg(Encode!(&args).unwrap())
    .call().await.unwrap();

  Decode!(&response_blob, Allowance).unwrap()
}