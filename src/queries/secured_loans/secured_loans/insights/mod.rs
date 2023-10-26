use crate::fields::insight_data::{InsightField, SecuredLoan};
use juniper::GraphQLObject;

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(name = "SecuredLoansInsights", description = "")]
pub struct Insights {
    pub count: i32,
}

impl Insights {
    pub fn new(insight: Vec<&InsightField<SecuredLoan>>) -> Self {
        Insights {
            count: insight.len() as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_get_secured_loans_insights() {
        let query = r#"
        query Insights {
            reports {
              report {
                secured_loans {
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
                    "secured_loans": {
                      "insights": {
                        "count": 2
                      }
                    }
                  },
                  {
                    "secured_loans": {
                      "insights": {
                        "count": 2
                      }
                    }
                  }
                ]
              }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}
