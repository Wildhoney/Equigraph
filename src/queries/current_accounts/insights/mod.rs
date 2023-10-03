use crate::schema::Context;

use super::fields;

#[derive(Debug, PartialEq)]
pub struct CurrentAccountInsights<'a> {
    pub accounts: Vec<&'a fields::CurrentAccount>,
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
