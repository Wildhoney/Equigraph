use super::fields::{self};
use super::{changes::CurrentAccountChanges, fields::CurrentAccountField};
use crate::objects::input::{Select, Since};
use crate::objects::output::Balance;
use crate::{parser::fields::PaymentStatusField, schema::Context};

#[derive(Debug, PartialEq)]
pub struct CurrentAccountPaymentHistory<'a> {
    pub select: Option<Select>,
    pub account: &'a CurrentAccountField,
    pub payment_history: &'a fields::PaymentHistoryField,
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountPaymentHistory<'_> {
    #[graphql(name = "age_in_months")]
    pub fn age_in_months(&self) -> i32 {
        self.payment_history.age_in_months
    }

    #[graphql(name = "payment_status")]
    pub fn payment_status(&self) -> &PaymentStatusField {
        &self.payment_history.payment_status
    }

    #[graphql(name = "account_balance")]
    pub fn account_balance(&self) -> Balance {
        Balance {
            amount: self.payment_history.account_balance.balance_amount.amount,
            currency: &self.payment_history.account_balance.balance_amount.currency,
        }
    }

    pub fn changes(&self, since: Since) -> CurrentAccountChanges {
        CurrentAccountChanges {
            since: since.to_owned(),
            account: &self.account,
            payment_history: &self.payment_history,
        }
    }
}
