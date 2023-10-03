use juniper::GraphQLEnum;

#[derive(Debug, PartialEq, GraphQLEnum, Clone)]
pub enum Select {
    All,
    Latest,
    Oldest,
    Polar,
}

#[derive(Debug, Clone, PartialEq, GraphQLEnum)]
pub enum Since {
    Previous,
    Next,
    First,
}