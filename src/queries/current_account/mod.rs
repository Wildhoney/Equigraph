pub mod types;
mod utils;

use crate::{
    schema::Context,
    utils::{BalanceObject, FrequencyKind},
};

use self::{
    types::{CurrentAccountInsightRoot, CurrentAccountRoot, CurrentAccountsRoot},
    utils::get_accounts,
};

#[juniper::graphql_object(context = Context)]
impl CurrentAccountsRoot<'_> {
    pub fn list(&self) -> Vec<CurrentAccountRoot> {
        match self.report {
            Some(report) => get_accounts(report)
                .iter()
                .map(|current_account| CurrentAccountRoot {
                    account: &current_account,
                })
                .collect::<Vec<_>>(),
            None => vec![],
        }
    }

    pub fn insight(&self) -> Option<CurrentAccountInsightRoot> {
        match self.report {
            Some(report) => Some(CurrentAccountInsightRoot {
                accounts: get_accounts(report),
            }),
            None => None,
        }
    }
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountRoot<'_> {
    #[graphql(name = "account_number")]
    pub fn account_number(&self) -> &String {
        &self.account.account_number
    }

    #[graphql(name = "company_name")]
    pub fn company_name(&self) -> &String {
        &self.account.company_name
    }

    #[graphql(name = "has_overdraft")]
    pub fn has_overdraft(&self) -> &bool {
        &self.account.overdraft
    }

    #[graphql(name = "current_balance")]
    pub fn current_balance(&self) -> BalanceObject {
        BalanceObject {
            amount: self.account.current_balance.balance_amount.amount,
            currency: self
                .account
                .current_balance
                .balance_amount
                .currency
                .to_string(),
        }
    }

    #[graphql(name = "default_balance")]
    pub fn default_balance(&self) -> BalanceObject {
        BalanceObject {
            amount: self.account.default_balance.balance_amount.amount,
            currency: self
                .account
                .default_balance
                .balance_amount
                .currency
                .to_string(),
        }
    }

    #[graphql(name = "payment_frequency")]
    pub fn payment_frequency(&self) -> &FrequencyKind {
        &self.account.payment_frequency
    }
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountInsightRoot<'_> {
    pub fn count(&self) -> i32 {
        5
    }
}
