use serde::Deserialize;

use crate::{parser::types::Report, utils::FrequencyKind};

#[derive(Debug, PartialEq)]
pub struct CurrentAccountsRoot<'a> {
    pub report: Option<&'a Report>,
}

#[derive(Debug, PartialEq)]
pub struct CurrentAccountRoot<'a> {
    pub account: &'a CurrentAccount,
}

#[derive(Debug, PartialEq)]
pub struct CurrentAccountInsightRoot<'a> {
    pub accounts: Vec<&'a CurrentAccount>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct CurrentAccount {
    #[serde(alias = "accountNumber")]
    pub account_number: String,
    #[serde(alias = "currentBalance")]
    pub current_balance: Balance,
    #[serde(alias = "defaultBalance")]
    pub default_balance: Balance,
    pub overdraft: bool,
    #[serde(alias = "paymentFrequency")]
    pub payment_frequency: FrequencyKind,
    #[serde(alias = "companyName")]
    pub company_name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Balance {
    #[serde(alias = "balanceAmount")]
    pub balance_amount: Amount,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Amount {
    pub amount: i32,
    pub currency: String,
}
