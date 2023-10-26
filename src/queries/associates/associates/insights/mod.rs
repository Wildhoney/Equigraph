use crate::queries::associates::associate::AssociateField;
use juniper::GraphQLObject;

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(name = "AssociatesInsights", description = "")]
pub struct Insights {
    pub count: i32,
}

impl Insights {
    pub fn new(associates: Vec<AssociateField>) -> Self {
        Insights {
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
        query Insights {
            reports {
              report {
                associates {
                  insights {
                    count
                  }
                }
              }
            }
          }
        "#;

        let expected = graphql_value!({
            "reports": {
                "report": [
                  {
                    "associates": {
                      "insights": {
                        "count": 1
                      }
                    }
                  },
                  {
                    "associates": {
                      "insights": {
                        "count": 1
                      }
                    }
                  }
                ]
              }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}
