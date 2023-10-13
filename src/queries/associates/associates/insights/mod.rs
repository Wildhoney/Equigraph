use crate::queries::associates::associate::AssociateField;
use juniper::GraphQLObject;

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(description = "")]
pub struct AssociatesInsights {
    pub count: i32,
}

impl AssociatesInsights {
    pub fn new(associates: Vec<AssociateField>) -> Self {
        AssociatesInsights {
            count: associates.len() as i32,
        }
    }
}
