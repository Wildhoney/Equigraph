pub mod types;
mod utils;

use crate::{
    fields,
    objects::{
        self,
        input::Since,
        output::{Impact, Polarity},
    },
    schema::Context,
};

use self::{
    types::{
        Company, CurrentAccountChangeObject, CurrentAccountInsightObject, CurrentAccountObject,
        CurrentAccountsObject, PaymentHistory,
    },
    utils::{get_accounts, get_delta, get_payment_history},
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
        let mut payment_history = get_payment_history(&self.account.payment_history).into_iter();

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

    pub fn change(&self, since: Since) -> CurrentAccountChangeObject {
        CurrentAccountChangeObject {
            current_index: 0,
            since: since.to_owned(),
            account: &self.account,
        }
    }
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountInsightObject<'_> {
    pub fn count(&self) -> i32 {
        self.accounts.len() as i32
    }
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountChangeObject<'_> {
    pub fn delta(&self) -> Option<i32> {
        get_delta(
            self.current_index,
            &self.since,
            &self.account.payment_history,
        )
    }

    pub fn impact(&self) -> Option<Impact> {
        let delta = get_delta(
            self.current_index,
            &self.since,
            &self.account.payment_history,
        );

        match delta {
            Some(delta) => match delta {
                delta if delta > 1_000 => Some(Impact::High),
                delta if delta > 500 => Some(Impact::Low),
                _ => Some(Impact::None),
            },
            _ => None,
        }
    }

    pub fn polarity(&self) -> Option<Polarity> {
        let delta = get_delta(
            self.current_index,
            &self.since,
            &self.account.payment_history,
        );

        match delta {
            Some(delta) => match delta {
                delta if delta == 0 => Some(Polarity::Unchanged),
                delta if delta > 0 => Some(Polarity::Negative),
                delta if delta < 500 => Some(Polarity::Positive),
                _ => Some(Polarity::Unchanged),
            },
            _ => None,
        }
    }
}
