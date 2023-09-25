use juniper::{GraphQLEnum, GraphQLObject};

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
    #[graphql(name = "A_YEAR_AGO")]
    AYearAgo,
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
