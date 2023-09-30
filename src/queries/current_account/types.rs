use juniper::GraphQLObject;

use crate::fields;

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

#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct Company<'a> {
    pub kind: &'a fields::CompanyClass,
    pub name: &'a String,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct PaymentHistory<'a> {
    pub balance: &'a fields::current_account::Balance,
    #[graphql(name = "age_in_months")]
    pub age_in_months: i32,
    #[graphql(name = "payment_status")]
    pub payment_status: &'a fields::PaymentStatus,
}
