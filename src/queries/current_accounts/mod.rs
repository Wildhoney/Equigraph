mod changes;
mod current_account;
pub mod fields;
mod insights;
mod payment_history;
mod utils;

use self::{current_account::CurrentAccount, insights::CurrentAccountInsights};
use crate::{parser::fields::ReportField, schema::Context};
use juniper::FieldResult;

pub fn fetch(context: &Context) -> FieldResult<CurrentAccounts> {
    Ok(CurrentAccounts {
        report: context.reports.get(0),
    })
}

#[derive(Debug, PartialEq)]
pub struct CurrentAccounts<'a> {
    pub report: Option<&'a ReportField>,
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccounts<'_> {
    #[graphql(name = "current_account")]
    pub fn current_account(&self) -> Vec<CurrentAccount> {
        current_account::fetch(self.report)
    }

    pub fn insights(&self) -> Option<CurrentAccountInsights> {
        insights::fetch(self.report)
    }
}
