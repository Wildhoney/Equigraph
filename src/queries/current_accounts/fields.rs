use juniper::GraphQLObject;
use serde::Deserialize;

use crate::{
    objects,
    parser::fields::{PaymentFrequencyField, PaymentStatusField},
};

#[derive(Debug, PartialEq, Deserialize)]
pub struct CurrentAccountField {
    #[serde(alias = "accountNumber")]
    pub account_number: String,
    #[serde(alias = "currentBalance")]
    pub current_balance: BalanceField,
    #[serde(alias = "defaultBalance")]
    pub default_balance: BalanceField,
    #[serde(alias = "startBalance")]
    pub start_balance: BalanceField,
    pub overdraft: bool,
    #[serde(alias = "paymentFrequency")]
    pub payment_frequency: PaymentFrequencyField,
    #[serde(alias = "companyName")]
    pub company_name: String,
    #[serde(alias = "companyClass")]
    pub company_class: objects::output::CompanyClass,
    #[serde(alias = "paymentHistory")]
    pub payment_history: Vec<PaymentHistoryField>,
}

// #[derive(Debug, GraphQLObject, Clone)]
// #[graphql(description = "")]
// pub struct PaymentField<'a> {
//     pub status: &'a PaymentStatusField,
//     pub frequency: &'a PaymentFrequencyField,
// }

#[derive(Debug, PartialEq, Deserialize, GraphQLObject)]
pub struct BalanceField {
    #[serde(alias = "balanceAmount")]
    pub balance_amount: AmountField,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLObject)]
pub struct AmountField {
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
    pub payment_status: PaymentStatusField,
}
