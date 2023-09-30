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

#[derive(Debug, GraphQLObject, Clone)]
#[graphql(description = "")]
pub struct Company<'a> {
    pub kind: &'a fields::CompanyClass,
    pub name: &'a String,
}
