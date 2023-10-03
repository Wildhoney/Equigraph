use crate::{fields, objects, schema::Context};

use super::changes::CurrentAccountChanges;

#[derive(Debug, PartialEq)]
pub struct CurrentAccountPaymentHistory<'a> {
    pub select: Option<objects::input::Select>,
    pub account: &'a fields::current_account::CurrentAccount,
    pub payment_history: &'a fields::current_account::PaymentHistory,
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountPaymentHistory<'_> {
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

    pub fn changes(&self, since: objects::input::Since) -> CurrentAccountChanges {
        CurrentAccountChanges {
            since: since.to_owned(),
            account: &self.account,
            payment_history: &self.payment_history,
        }
    }
}
