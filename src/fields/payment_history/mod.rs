mod changes;
mod utils;

use self::{changes::PaymentHistoryChanges, utils::select_payment_history};
use super::{AmountField, BalanceField, PaymentStatusField};
use crate::{
    objects::input::{Select, Since},
    schema::Context,
    utils::unique_id,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct PaymentHistoryField {
    #[serde(default = "unique_id")]
    pub id: Uuid,
    #[serde(alias = "accountBalance")]
    pub account_balance: BalanceField,
    #[serde(alias = "ageInMonths")]
    pub age_in_months: i32,
    #[serde(alias = "paymentStatus")]
    pub payment_status: PaymentStatusField,
}

pub trait PartitionPaymentHistory {
    fn partition(&self, select: Select) -> &[PaymentHistoryField];
}

impl PartitionPaymentHistory for Vec<PaymentHistoryField> {
    fn partition(&self, select: Select) -> &[PaymentHistoryField] {
        select_payment_history(select, self)
    }
}

#[juniper::graphql_object(context = Context)]
impl PaymentHistoryField {
    #[graphql(name = "age_in_months")]
    pub fn age_in_months(&self) -> i32 {
        self.age_in_months
    }

    #[graphql(name = "payment_status")]
    pub fn payment_status(&self) -> &PaymentStatusField {
        &self.payment_status
    }

    #[graphql(name = "account_balance")]
    pub fn account_balance(&self) -> &AmountField {
        &self.account_balance.balance_amount
    }

    pub fn changes(&self, context: &Context, since: Since) -> Option<PaymentHistoryChanges> {
        PaymentHistoryChanges::new(
            since,
            self.id,
            self.account_balance.balance_amount.amount as u32,
            &context.reports,
        )
    }
}
