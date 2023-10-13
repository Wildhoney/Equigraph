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

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_get_associates_insights() {
        let query = r#"
            query AssociatesInsights {
                associates {
                    insights {
                        count
                    }
                }
            }
        "#;

        let expected = graphql_value!({
            "associates": {
                "insights": {
                    "count": 2
                }
            }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}
