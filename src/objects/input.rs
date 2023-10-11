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

pub type Format = String;
