use juniper::GraphQLObject;
use serde::Deserialize;

use crate::{
    parser::types::Report,
    utils::{CompanyKind, FrequencyKind, PaymentStatusKind},
};

#[derive(Debug, PartialEq)]
pub struct CurrentAccountsObject<'a> {
    pub report: Option<&'a Report>,
}

#[derive(Debug, PartialEq)]
pub struct CurrentAccountObject<'a> {
    pub account: &'a CurrentAccountField,
}

#[derive(Debug, PartialEq)]
pub struct CurrentAccountInsightObject<'a> {
    pub accounts: Vec<&'a CurrentAccountField>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct CurrentAccountField {
    #[serde(alias = "accountNumber")]
    pub account_number: String,
    #[serde(alias = "currentBalance")]
    pub current_balance: BalanceField,
    #[serde(alias = "defaultBalance")]
    pub default_balance: BalanceField,
    pub overdraft: bool,
    #[serde(alias = "paymentFrequency")]
    pub payment_frequency: FrequencyKind,
    #[serde(alias = "companyName")]
    pub company_name: String,
    #[serde(alias = "companyClass")]
    pub company_class: CompanyKind,
    #[serde(alias = "paymentHistory")]
    pub payment_history: Vec<PaymentHistoryField>,
}

#[derive(Debug, GraphQLObject, Clone)]
#[graphql(description = "")]
pub struct Company<'a> {
    pub kind: &'a CompanyKind,
    pub name: &'a String,
}

#[derive(Debug, GraphQLObject, Clone)]
#[graphql(description = "")]
pub struct Payment<'a> {
    pub status: &'a PaymentStatusKind,
    pub frequency: &'a FrequencyKind,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct BalanceField {
    #[serde(alias = "balanceAmount")]
    pub balance_amount: Amount,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Amount {
    pub amount: i32,
    pub currency: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct PaymentHistoryField {
    #[serde(alias = "accountBalance")]
    pub account_balance: BalanceField,
    #[serde(alias = "ageInMonths")]
    pub age_in_months: i32,
    #[serde(alias = "paymentStatus")]
    pub payment_status: PaymentStatusKind,
}
