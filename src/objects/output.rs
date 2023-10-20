use juniper::{GraphQLEnum, GraphQLObject};
use serde::Deserialize;

use crate::schema::Context;

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
    #[serde(alias = "RT")]
    WTF1,
    #[serde(alias = "FN")]
    WTF2,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct Company<'a> {
    pub kind: &'a CompanyClass,
    pub name: &'a String,
}

#[derive(Debug)]
pub struct PaymentAnalysis {
    pub active: bool,
    pub total: i32,
    pub payments: i32,
}

#[juniper::graphql_object(context = Context)]
impl PaymentAnalysis {
    pub fn made(&self) -> i32 {
        self.payments
    }

    pub fn remaining(&self) -> Option<i32> {
        if self.active == false {
            return None;
        }

        Some((self.total - self.payments as i32) as i32)
    }

    pub fn total(&self) -> Option<i32> {
        if self.active == false {
            return None;
        }

        Some(self.total)
    }
}
