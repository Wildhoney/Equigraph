mod changes;
mod current_account;
pub mod fields;
mod insights;
mod payment_history;
mod utils;

use self::{current_account::CurrentAccount, insights::CurrentAccountInsights};
use crate::{parser::fields::ReportField, schema::Context};
use juniper::FieldResult;

#[derive(Debug, PartialEq)]
pub struct CurrentAccounts<'a> {
    pub report: Option<&'a ReportField>,
}

impl CurrentAccounts<'_> {
    pub fn new(context: &Context) -> FieldResult<CurrentAccounts> {
        Ok(CurrentAccounts {
            report: context.reports.get(0),
        })
    }
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccounts<'_> {
    #[graphql(name = "current_account")]
    pub fn current_account(&self) -> Vec<CurrentAccount> {
        CurrentAccount::new(self.report)
    }

    pub fn insights(&self) -> Option<CurrentAccountInsights> {
        CurrentAccountInsights::new(self.report)
    }
}
