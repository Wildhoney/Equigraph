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

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_display_insights() {
        let query = r#"
            query Associates {
                associates {
                  insights {
                    count
                  }
                }
              }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "associates": {"insights": { "count": 1 }}
            })
        );
    }
}
