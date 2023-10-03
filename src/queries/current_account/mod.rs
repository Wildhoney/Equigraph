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
        CurrentAccountChangeObject, CurrentAccountInsightObject, CurrentAccountObject,
        CurrentAccountPaymentHistoryObject, CurrentAccountsObject,
    },
    utils::{get_accounts, get_delta, get_impact, get_polarity},
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

    pub fn company(&self) -> objects::output::Company {
        objects::output::Company {
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
    pub fn payment_history(
        &self,
        select: Option<objects::input::Select>,
    ) -> Vec<CurrentAccountPaymentHistoryObject> {
        let mut payment_histories = self.account.payment_history.iter();

        let payment_history = match select {
            Some(objects::input::Select::Latest) => match payment_histories.nth(0) {
                Some(payment_history) => vec![payment_history],
                _ => vec![],
            },
            Some(objects::input::Select::Oldest) => match payment_histories.last() {
                Some(payment_history) => vec![payment_history],
                _ => vec![],
            },
            Some(objects::input::Select::Polar) => {
                match [payment_histories.nth(0), payment_histories.last()] {
                    [Some(first), Some(last)] => vec![first, last],
                    _ => vec![],
                }
            }
            _ => payment_histories.collect::<Vec<_>>(),
        };

        payment_history
            .iter()
            .map(|payment_history| CurrentAccountPaymentHistoryObject {
                select: select.to_owned(),
                account: &self.account,
                payment_history: payment_history,
            })
            .collect::<Vec<_>>()
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
        get_delta(&self.since, &self.account, &self.payment_history)
    }

    pub fn impact(&self) -> Option<Impact> {
        get_impact(&self.since, &self.account, &self.payment_history)
    }

    pub fn polarity(&self) -> Option<Polarity> {
        get_polarity(&self.since, &self.account, &self.payment_history)
    }
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountPaymentHistoryObject<'_> {
    #[graphql(name = "age_in_months")]
    pub fn age_in_months(&self) -> i32 {
        self.payment_history.age_in_months
    }

    #[graphql(name = "account_balance")]
    pub fn account_balance(&self) -> objects::output::Balance {
        objects::output::Balance {
            amount: self.payment_history.account_balance.balance_amount.amount,
            currency: self
                .payment_history
                .account_balance
                .balance_amount
                .currency
                .to_string(),
        }
    }

    pub fn change(&self, since: Since) -> CurrentAccountChangeObject {
        CurrentAccountChangeObject {
            since: since.to_owned(),
            account: &self.account,
            payment_history: &self.payment_history,
        }
    }
}
