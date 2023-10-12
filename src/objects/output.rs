use juniper::{GraphQLEnum, GraphQLObject};
use serde::Deserialize;

pub type Date = String;

#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct Name<'a> {
    pub title: &'a str,
    pub forename: &'a str,
    pub surname: &'a str,
}

#[derive(Debug, PartialEq, GraphQLEnum)]
pub enum Sentiment {
    High,
    Medium,
    Low,
}

#[derive(Debug, PartialEq, GraphQLEnum)]
pub enum Polarity {
    Unchanged,
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, GraphQLEnum)]
pub enum Impact {
    None,
    High,
    Low,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum, Clone)]
pub enum CompanyClass {
    #[serde(alias = "BK")]
    Bank,
    #[serde(alias = "MS")]
    MortgageSupplier,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct Company<'a> {
    pub kind: &'a CompanyClass,
    pub name: &'a String,
}
