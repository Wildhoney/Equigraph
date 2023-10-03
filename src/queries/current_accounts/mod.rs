mod changes;
mod current_account;
mod insights;
mod payment_history;
mod utils;

use crate::{fields::Report, schema::Context};

use self::{
    current_account::CurrentAccount, insights::CurrentAccountInsights, utils::get_accounts,
};

#[derive(Debug, PartialEq)]
pub struct CurrentAccounts<'a> {
    pub report: Option<&'a Report>,
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccounts<'_> {
    #[graphql(name = "current_account")]
    pub fn current_account(&self) -> Vec<CurrentAccount> {
        match self.report {
            Some(report) => get_accounts(report)
                .iter()
                .map(|current_account| CurrentAccount {
                    account: &current_account,
                })
                .collect::<Vec<_>>(),
            None => vec![],
        }
    }

    pub fn insights(&self) -> Option<CurrentAccountInsights> {
        match self.report {
            Some(report) => Some(CurrentAccountInsights {
                accounts: get_accounts(report),
            }),
            None => None,
        }
    }
}
