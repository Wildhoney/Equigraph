use juniper::{GraphQLEnum, GraphQLObject};

#[derive(GraphQLEnum)]
pub enum Polarity {
    Unchanged,
    Positive,
    Negative,
}

#[derive(GraphQLEnum)]
pub enum Impact {
    None,
    High,
    Low,
}

#[derive(GraphQLEnum)]
pub enum Since {
    Previous,
    #[graphql(name = "A_YEAR_AGO")]
    AYearAgo,
}

#[derive(GraphQLObject)]
#[graphql(description = "")]
pub struct DateObject {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

#[derive(GraphQLObject)]
#[graphql(description = "")]
pub struct NameObject {
    pub title: String,
    pub forename: String,
    pub surname: String,
}
