use super::{fields::CurrentAccountField, utils::get_accounts};
use crate::{parser::types::Report, schema::Context};

pub fn fetch<'a>(report: Option<&'a Report>) -> Option<CurrentAccountInsights> {
    match report {
        Some(report) => Some(CurrentAccountInsights {
            accounts: get_accounts(report),
        }),
        None => None,
    }
}

#[derive(Debug, PartialEq)]
pub struct CurrentAccountInsights<'a> {
    pub accounts: Vec<&'a CurrentAccountField>,
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountInsights<'_> {
    #[graphql(name = "accounts_count")]
    pub fn accounts_count(&self) -> i32 {
        self.accounts.len() as i32
    }

    #[graphql(name = "has_overdraft")]
    pub fn has_overdraft(&self) -> bool {
        self.accounts.iter().any(|account| account.overdraft)
    }
}
