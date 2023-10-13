use crate::schema::Context;
use juniper::GraphQLObject;

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(description = "")]
pub struct AssociatesInsights {
    pub count: i32,
}

impl AssociatesInsights {
    pub fn new(context: &Context) -> Self {
        AssociatesInsights {
            count: context
                .reports
                .iter()
                .map(|report| &report.non_address_specific_data.associates)
                .collect::<Vec<_>>()
                .len() as i32,
        }
    }
}
