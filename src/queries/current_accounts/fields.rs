use juniper::GraphQLObject;
use serde::Deserialize;

use crate::{
    objects::output::CompanyClass,
    parser::fields::{DateField, PaymentFrequencyField, PaymentStatusField},
};

#[derive(Debug, PartialEq, Deserialize)]
pub struct CurrentAccountField {
    #[serde(alias = "accountNumber")]
    pub account_number: String,
    #[serde(alias = "currentBalance")]
    pub current_balance: BalanceField,
    #[serde(alias = "creditLimit")]
    pub credit_limit: Option<CreditLimitField>,
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
    pub company_class: CompanyClass,
    #[serde(alias = "paymentHistory")]
    pub payment_history: Vec<PaymentHistoryField>,
    #[serde(alias = "startDate")]
    pub start_date: DateField,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLObject)]
pub struct BalanceField {
    #[serde(alias = "balanceAmount")]
    pub balance_amount: AmountField,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLObject)]
pub struct CreditLimitField {
    pub limit: AmountField,
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
