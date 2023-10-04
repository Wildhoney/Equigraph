use super::{fields::CurrentAccountField, utils::get_accounts};
use crate::{parser::types::Report, schema::Context};

#[derive(Debug, PartialEq)]
pub struct CurrentAccountInsights<'a> {
    pub accounts: Vec<&'a CurrentAccountField>,
}

impl CurrentAccountInsights<'_> {
    pub fn new<'a>(report: Option<&'a Report>) -> Option<CurrentAccountInsights> {
        match report {
            Some(report) => Some(CurrentAccountInsights {
                accounts: get_accounts(report),
            }),
            None => None,
        }
    }
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
