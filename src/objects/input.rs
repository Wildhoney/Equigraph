use juniper::GraphQLEnum;

#[derive(Debug, PartialEq, GraphQLEnum, Clone)]
pub enum Select {
    All,
    Latest,
    Oldest,
}

#[derive(Debug, Clone, PartialEq, GraphQLEnum)]
pub enum Since {
    First,
    Previous,
    Next,
    Last,
}

pub type Like = String;

#[derive(Debug, Clone, PartialEq, GraphQLEnum)]
#[graphql(description = "Whether to strip the ending zeroes from the amount")]
pub enum EndingZeroes {
    Strip,
    Keep,
}

#[derive(Debug, Clone, PartialEq, GraphQLEnum)]
#[graphql(description = "Whether to enforce the uniqueness of the records")]
pub enum Unique {
    Yes,
    No,
}

#[derive(Debug, Clone, PartialEq, GraphQLEnum)]
#[graphql(description = "Whether to include or exclude the active records")]
pub enum Active {
    Include,
    Exclude,
}

#[derive(Debug)]
pub struct QueryOptions {
    pub unique: Option<Unique>,
    pub active: Option<Active>,
}
