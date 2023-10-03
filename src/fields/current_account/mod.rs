use juniper::GraphQLObject;
use serde::Deserialize;

use crate::objects;

use super::{PaymentFrequency, PaymentStatus};

#[derive(Debug, PartialEq, Deserialize)]
pub struct CurrentAccount {
    #[serde(alias = "accountNumber")]
    pub account_number: String,
    #[serde(alias = "currentBalance")]
    pub current_balance: Balance,
    #[serde(alias = "defaultBalance")]
    pub default_balance: Balance,
    #[serde(alias = "startBalance")]
    pub start_balance: Balance,
    pub overdraft: bool,
    #[serde(alias = "paymentFrequency")]
    pub payment_frequency: PaymentFrequency,
    #[serde(alias = "companyName")]
    pub company_name: String,
    #[serde(alias = "companyClass")]
    pub company_class: objects::output::CompanyClass,
    #[serde(alias = "paymentHistory")]
    pub payment_history: Vec<PaymentHistory>,
}

#[derive(Debug, GraphQLObject, Clone)]
#[graphql(description = "")]
pub struct Payment<'a> {
    pub status: &'a PaymentStatus,
    pub frequency: &'a PaymentFrequency,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLObject)]
pub struct Balance {
    #[serde(alias = "balanceAmount")]
    pub balance_amount: Amount,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLObject)]
pub struct Amount {
    pub amount: i32,
    pub currency: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct PaymentHistory {
    #[serde(alias = "accountBalance")]
    pub account_balance: Balance,
    #[serde(alias = "ageInMonths")]
    pub age_in_months: i32,
    #[serde(alias = "paymentStatus")]
    pub payment_status: PaymentStatus,
}
