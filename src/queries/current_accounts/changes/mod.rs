mod utils;

use crate::{fields, objects, schema::Context};

use self::utils::{get_delta, get_impact, get_polarity};

#[derive(Debug, PartialEq)]
pub struct CurrentAccountChanges<'a> {
    pub since: objects::input::Since,
    pub account: &'a fields::current_account::CurrentAccount,
    pub payment_history: &'a fields::current_account::PaymentHistory,
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
