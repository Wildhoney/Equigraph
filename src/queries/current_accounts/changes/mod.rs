mod utils;

use self::utils::{get_delta, get_impact, get_polarity};
use super::fields::{CurrentAccountField, PaymentHistoryField};
use crate::{
    objects::{self, input::Since},
    schema::Context,
};

#[derive(Debug, PartialEq)]
pub struct CurrentAccountChanges<'a> {
    pub since: Since,
    pub account: &'a CurrentAccountField,
    pub payment_history: &'a PaymentHistoryField,
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountChanges<'_> {
    pub fn delta(&self) -> Option<i32> {
        get_delta(&self.since, &self.account, &self.payment_history)
    }

    pub fn impact(&self) -> Option<objects::output::Impact> {
        get_impact(&self.since, &self.account, &self.payment_history)
    }

    pub fn polarity(&self) -> Option<objects::output::Polarity> {
        get_polarity(&self.since, &self.account, &self.payment_history)
    }
}
