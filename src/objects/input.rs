use juniper::GraphQLEnum;

#[derive(Debug, PartialEq, GraphQLEnum, Clone)]
pub enum Select {
    All,
    Latest,
    Oldest,
}

#[derive(Debug, Clone, PartialEq, GraphQLEnum)]
pub enum Since {
    Previous,
    Next,
    First,
}

pub type Format = String;
