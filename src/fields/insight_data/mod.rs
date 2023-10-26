pub mod changes;
pub mod traits;
pub mod utils;

use crate::{objects::output::CompanyClass, utils::unique_id};
use serde::Deserialize;
use uuid::Uuid;

use super::{
    payment_history::PaymentHistoryField, BalanceField, CreditLimitField, DateField,
    FixedPaymentTermsField, LoanTypeField, PaymentFrequencyField,
};

#[derive(Debug)]
pub enum InsightKind<'a> {
    CurrentAccount(&'a InsightField<CurrentAccount>),
    SecuredLoan(&'a InsightField<SecuredLoan>),
    UnsecuredLoan(&'a InsightField<UnsecuredLoan>),
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct InsightDataField {
    #[serde(alias = "currentAccount")]
    pub current_account: Vec<InsightField<CurrentAccount>>,
    #[serde(alias = "securedLoan")]
    pub secured_loan: Vec<InsightField<SecuredLoan>>,
    #[serde(alias = "unsecuredLoan")]
    pub unsecured_loan: Vec<InsightField<UnsecuredLoan>>,
    // #[serde(alias = "creditCard")]
    // pub credit_card: Vec<CreditLimitField>,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
pub struct CurrentAccount;

#[derive(Debug, PartialEq, Deserialize, Default)]
pub struct SecuredLoan;

#[derive(Debug, PartialEq, Deserialize, Default)]
pub struct UnsecuredLoan;

#[derive(Debug, PartialEq, Deserialize, Default)]
pub struct CreditCard;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct InsightField<Kind> {
    #[serde(skip_deserializing, skip_serializing)]
    kind: Kind,
    #[serde(default = "unique_id")]
    pub id: Uuid,
    #[serde(alias = "accountNumber")]
    pub account_number: String,
    #[serde(alias = "currentBalance")]
    pub current_balance: BalanceField,
    #[serde(alias = "defaultBalance")]
    pub default_balance: BalanceField,
    #[serde(alias = "startBalance")]
    pub start_balance: BalanceField,
    #[serde(alias = "paymentHistory")]
    pub payment_history: Vec<PaymentHistoryField>,
    #[serde(alias = "loanType")]
    pub loan_type: Option<LoanTypeField>,
    #[serde(alias = "startDate")]
    pub start_date: DateField,
    #[serde(alias = "lastUpdateDate")]
    pub last_update_date: DateField,
    #[serde(alias = "endDate")]
    pub end_date: Option<DateField>,
    #[serde(alias = "paymentFrequency")]
    pub payment_frequency: PaymentFrequencyField,
    #[serde(alias = "companyName")]
    pub company_name: String,
    #[serde(alias = "companyClass")]
    pub company_class: CompanyClass,
    pub flexible: Option<bool>,
    #[serde(alias = "fixedPaymentTerms")]
    pub fixed_payment_terms: Option<FixedPaymentTermsField>,
    pub overdraft: Option<bool>,
    #[serde(alias = "creditLimit")]
    pub credit_limit: Option<CreditLimitField>,
}
