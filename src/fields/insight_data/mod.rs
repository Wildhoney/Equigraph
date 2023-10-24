pub mod changes;
pub mod traits;
pub mod utils;

use crate::queries::{
    current_accounts::current_account::CurrentAccountField,
    secured_loans::secured_loan::SecuredLoanField,
    unsecured_loans::unsecured_loan::UnsecuredLoanField,
};
use serde::Deserialize;

#[derive(Debug)]
pub enum InsightKind<'a> {
    CurrentAccount(&'a CurrentAccountField),
    SecuredLoan(&'a SecuredLoanField),
    UnsecuredLoan(&'a UnsecuredLoanField),
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct InsightDataField {
    #[serde(alias = "currentAccount")]
    pub current_account: Vec<CurrentAccountField>,
    #[serde(alias = "securedLoan")]
    pub secured_loan: Vec<SecuredLoanField>,
    #[serde(alias = "unsecuredLoan")]
    pub unsecured_loan: Vec<UnsecuredLoanField>,
}
