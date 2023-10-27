pub mod changes;
pub mod traits;
pub mod utils;

use crate::{objects::output::CompanyClass, utils::unique_id};
use itertools::Itertools;
use serde::Deserialize;
use uuid::Uuid;

use super::{
    payment_history::PaymentHistoryField, BalanceField, CreditLimitField, DateField,
    FixedPaymentTermsField, LoanTypeField, PaymentFrequencyField,
};

#[derive(Debug)]
pub enum InsightVariant<'a> {
    CurrentAccount(&'a InsightField<CurrentAccount>),
    SecuredLoan(&'a InsightField<SecuredLoan>),
    UnsecuredLoan(&'a InsightField<UnsecuredLoan>),
    CreditCard(&'a InsightField<CreditCard>),
}

impl InsightVariant<'_> {
    pub fn get_payment_history(&self) -> &Vec<PaymentHistoryField> {
        match self {
            InsightVariant::CurrentAccount(item) => &item.payment_history,
            InsightVariant::SecuredLoan(item) => &item.payment_history,
            InsightVariant::UnsecuredLoan(item) => &item.payment_history,
            InsightVariant::CreditCard(item) => &item.payment_history,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct InsightDataField {
    #[serde(alias = "currentAccount")]
    pub current_account: Vec<InsightField<CurrentAccount>>,
    #[serde(alias = "securedLoan")]
    pub secured_loan: Vec<InsightField<SecuredLoan>>,
    #[serde(alias = "unsecuredLoan")]
    pub unsecured_loan: Vec<InsightField<UnsecuredLoan>>,
    #[serde(alias = "creditCard")]
    pub credit_card: Vec<InsightField<CreditCard>>,
}

pub trait InsightsTrait {
    fn get_insights(&self) -> Vec<InsightVariant>;
}

impl InsightsTrait for InsightDataField {
    fn get_insights(&self) -> Vec<InsightVariant> {
        vec![
            self.secured_loan
                .iter()
                .map(|item| InsightVariant::SecuredLoan(item))
                .collect_vec(),
            self.unsecured_loan
                .iter()
                .map(|item| InsightVariant::UnsecuredLoan(item))
                .collect_vec(),
            self.current_account
                .iter()
                .map(|item| InsightVariant::CurrentAccount(item))
                .collect_vec(),
            self.credit_card
                .iter()
                .map(|item| InsightVariant::CreditCard(item))
                .collect_vec(),
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }
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
pub struct InsightField<InsightVariant> {
    #[serde(skip_deserializing, skip_serializing)]
    insight_variant: InsightVariant,
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
