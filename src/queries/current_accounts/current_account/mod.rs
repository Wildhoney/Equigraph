use super::{
    fields::CurrentAccountField,
    payment_history::{self, CurrentAccountPaymentHistory},
    utils::get_accounts,
};
use crate::{
    objects::{
        input::Select,
        output::{Balance, Company},
    },
    parser::{fields::PaymentFrequencyField, types::Report},
    schema::Context,
};

pub fn fetch<'a>(report: Option<&'a Report>) -> Vec<CurrentAccount> {
    match report {
        Some(report) => get_accounts(report)
            .iter()
            .map(|current_account| CurrentAccount {
                account: &current_account,
            })
            .collect::<Vec<_>>(),
        None => vec![],
    }
}

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
    pub fn current_balance(&self) -> Balance {
        Balance {
            amount: self.account.current_balance.balance_amount.amount,
            currency: &self.account.current_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "default_balance")]
    pub fn default_balance(&self) -> Balance {
        Balance {
            amount: self.account.default_balance.balance_amount.amount,
            currency: &self.account.default_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "start_balance")]
    pub fn start_balance(&self) -> Balance {
        Balance {
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
        payment_history::fetch(select, self.account)
    }
}
