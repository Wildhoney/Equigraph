use super::{fields::CurrentAccountField, payment_history::CurrentAccountPaymentHistory};
use crate::{
    objects::{self, input::Select},
    parser::fields::PaymentFrequencyField,
    schema::Context,
};

#[derive(Debug, PartialEq)]
pub struct CurrentAccount<'a> {
    pub account: &'a CurrentAccountField,
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccount<'_> {
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
            currency: &self.account.current_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "default_balance")]
    pub fn default_balance(&self) -> objects::output::Balance {
        objects::output::Balance {
            amount: self.account.default_balance.balance_amount.amount,
            currency: &self.account.default_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "start_balance")]
    pub fn start_balance(&self) -> objects::output::Balance {
        objects::output::Balance {
            amount: self.account.start_balance.balance_amount.amount,
            currency: &self.account.start_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "payment_frequency")]
    pub fn payment_frequency(&self) -> &PaymentFrequencyField {
        &self.account.payment_frequency
    }

    #[graphql(name = "payment_history")]
    pub fn payment_history(&self, select: Option<Select>) -> Vec<CurrentAccountPaymentHistory> {
        let mut payment_histories = self.account.payment_history.iter();

        let payment_history = match select {
            Some(Select::Latest) => match payment_histories.nth(0) {
                Some(payment_history) => vec![payment_history],
                _ => vec![],
            },
            Some(Select::Oldest) => match payment_histories.last() {
                Some(payment_history) => vec![payment_history],
                _ => vec![],
            },
            Some(Select::Polar) => match [payment_histories.nth(0), payment_histories.last()] {
                [Some(first), Some(last)] => vec![first, last],
                _ => vec![],
            },
            _ => payment_histories.collect::<Vec<_>>(),
        };

        payment_history
            .iter()
            .map(|payment_history| CurrentAccountPaymentHistory {
                select: select.to_owned(),
                account: &self.account,
                payment_history: payment_history,
            })
            .collect::<Vec<_>>()
    }
}
