mod utils;

use self::utils::{get_delta, get_impact, get_polarity};
use super::fields;
use crate::{objects, schema::Context};

#[derive(Debug, PartialEq)]
pub struct CurrentAccountChanges<'a> {
    pub since: objects::input::Since,
    pub account: &'a fields::CurrentAccount,
    pub payment_history: &'a fields::PaymentHistory,
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
