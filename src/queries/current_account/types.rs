use crate::{fields, objects};

use crate::objects::input::Since;
use crate::parser::types::Report;

#[derive(Debug, PartialEq)]
pub struct CurrentAccountsObject<'a> {
    pub report: Option<&'a Report>,
}

#[derive(Debug, PartialEq)]
pub struct CurrentAccountObject<'a> {
    pub account: &'a fields::current_account::CurrentAccount,
}

#[derive(Debug, PartialEq)]
pub struct CurrentAccountInsightObject<'a> {
    pub accounts: Vec<&'a fields::current_account::CurrentAccount>,
}

#[derive(Debug, PartialEq)]
pub struct CurrentAccountChangeObject<'a> {
    pub since: Since,
    pub account: &'a fields::current_account::CurrentAccount,
    pub payment_history: &'a fields::current_account::PaymentHistory,
}

#[derive(Debug, PartialEq)]
pub struct CurrentAccountPaymentHistoryObject<'a> {
    pub select: Option<objects::input::Select>,
    pub account: &'a fields::current_account::CurrentAccount,
    pub payment_history: &'a fields::current_account::PaymentHistory,
}
