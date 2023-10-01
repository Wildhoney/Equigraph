pub mod types;
mod utils;

use crate::{fields, objects, schema::Context};

use self::{
    types::{
        Company, CurrentAccountInsightObject, CurrentAccountObject, CurrentAccountsObject,
        PaymentHistory,
    },
    utils::get_accounts,
};

#[juniper::graphql_object(context = Context)]
impl CurrentAccountsObject<'_> {
    #[graphql(name = "current_account")]
    pub fn current_account(&self) -> Vec<CurrentAccountObject> {
        match self.report {
            Some(report) => get_accounts(report)
                .iter()
                .map(|current_account| CurrentAccountObject {
                    account: &current_account,
                })
                .collect::<Vec<_>>(),
            None => vec![],
        }
    }

    pub fn insight(&self) -> Option<CurrentAccountInsightObject> {
        match self.report {
            Some(report) => Some(CurrentAccountInsightObject {
                accounts: get_accounts(report),
            }),
            None => None,
        }
    }
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountObject<'_> {
    #[graphql(name = "account_number")]
    pub fn account_number(&self) -> &String {
        &self.account.account_number
    }

    pub fn company(&self) -> Company {
        Company {
            kind: &self.account.company_class,
            name: &self.account.company_name,
        }
    }

    #[graphql(name = "has_overdraft")]
    pub fn has_overdraft(&self) -> &bool {
        &self.account.overdraft
    }

    #[graphql(name = "current_balance")]
    pub fn current_balance(&self) -> objects::output::Balance {
        objects::output::Balance {
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
    pub fn default_balance(&self) -> objects::output::Balance {
        objects::output::Balance {
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
    pub fn payment_frequency(&self) -> &fields::PaymentFrequency {
        &self.account.payment_frequency
    }

    #[graphql(name = "payment_history")]
    pub fn payment_history(&self, select: Option<objects::input::Select>) -> Vec<PaymentHistory> {
        let payment_history = self
            .account
            .payment_history
            .iter()
            .map(|payment_history| PaymentHistory {
                balance: &payment_history.account_balance,
                age_in_months: payment_history.age_in_months,
                payment_status: &payment_history.payment_status,
            })
            .collect::<Vec<_>>();

        let mut payment_history = payment_history.into_iter();

        match select {
            Some(objects::input::Select::Latest) => match payment_history.nth(0) {
                Some(payment_history) => vec![payment_history],
                _ => vec![],
            },
            Some(objects::input::Select::Oldest) => match payment_history.last() {
                Some(payment_history) => vec![payment_history],
                _ => vec![],
            },
            Some(objects::input::Select::Polar) => {
                match [payment_history.nth(0), payment_history.last()] {
                    [Some(first), Some(last)] => vec![first, last],
                    _ => vec![],
                }
            }
            _ => payment_history.collect::<Vec<_>>(),
        }
    }
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountInsightObject<'_> {
    pub fn count(&self) -> i32 {
        self.accounts.len() as i32
    }
}
