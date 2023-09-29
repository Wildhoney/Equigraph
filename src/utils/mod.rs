use juniper::{GraphQLEnum, GraphQLObject};
use serde::Deserialize;

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

#[derive(Debug, GraphQLEnum)]
pub enum Since {
    Previous,
    Next,
    First,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct DateObject {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct NameObject {
    pub title: String,
    pub forename: String,
    pub surname: String,
}

#[derive(Debug, PartialEq, GraphQLEnum)]
pub enum Sentiment {
    High,
    Medium,
    Low,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct BalanceObject {
    pub amount: i32,
    pub currency: String,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum)]
pub enum FrequencyKind {
    #[serde(alias = "MONTHLY")]
    Monthly,
    #[serde(alias = "PERIODICALLY")]
    Periodically,
}
