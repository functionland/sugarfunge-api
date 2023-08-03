use crate::primitives::*;
use serde::{Deserialize, Serialize};

// SBP-M1 review: hardcoded values seem problematic when progressing beyond dev-chain as dev accounts are well known
pub const REFUND_SEED: &str = "//Alice";
pub const REFUND_FEE_VALUE: u128 = 20000000000000000;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAccountOutput {
    // SBP-M1 review: remove seed
    pub seed: Seed,
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundAccountInput {
    // SBP-M1 review: remove seed
    pub seed: Seed,
    pub to: Account,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundAccountOutput {
    pub from: Account,
    pub to: Account,
    pub amount: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalanceInput {
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalanceOutput {
    pub balance: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountExistsInput {
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountExistsOutput {
    pub account: Account,
    pub exists: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeededAccountInput {
    // SBP-M1 review: remove seed
    pub seed: Seed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeededAccountOutput {
    pub seed: Seed,
    pub account: Account,
}
