use juniper::GraphQLEnum;

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
