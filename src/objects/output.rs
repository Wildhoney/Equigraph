use juniper::{GraphQLEnum, GraphQLObject};

#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct Date {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct Name {
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

#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct Balance {
    pub amount: i32,
    pub currency: String,
}
