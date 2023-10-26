mod changes;
pub mod utils;

use self::{changes::Changes, utils::select_payment_history};
use super::{AmountField, BalanceField, PaymentStatusField};
use crate::{
    objects::{
        input::{Select, Since},
        output::CreditLimitChange,
    },
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
    pub statement: Option<StatementField>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct StatementField {
    pub cash_advance_count: i32,
    pub cash_advance_value: AmountField,
    pub credit_limit_change: CreditLimitChange,
    pub payment_amount: AmountField,
    pub promotional_rate: bool,
    pub statement_balance: BalanceField,
}

#[juniper::graphql_object(context = Context)]
impl StatementField {
    #[graphql(name = "cash_advance_count")]
    pub fn cash_advance_count(&self) -> i32 {
        self.cash_advance_count
    }

    #[graphql(name = "cash_advance_value")]
    pub fn cash_advance_value(&self) -> &AmountField {
        &self.cash_advance_value
    }

    #[graphql(name = "credit_limit_change")]
    pub fn credit_limit_change(&self) -> &CreditLimitChange {
        &self.credit_limit_change
    }

    #[graphql(name = "payment_amount")]
    pub fn payment_amount(&self) -> &AmountField {
        &self.payment_amount
    }

    #[graphql(name = "is_promotional_rate")]
    pub fn is_promotional_rate(&self) -> bool {
        self.promotional_rate
    }

    #[graphql(name = "statement_balance")]
    pub fn statement_balance(&self) -> &AmountField {
        &self.statement_balance.balance_amount
    }
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

    pub fn statement(&self) -> &Option<StatementField> {
        &self.statement
    }

    pub fn changes(&self, context: &Context, since: Since) -> Option<Changes> {
        Changes::new(
            since,
            self.id,
            &self.account_balance.balance_amount,
            &context.reports,
        )
    }
}
