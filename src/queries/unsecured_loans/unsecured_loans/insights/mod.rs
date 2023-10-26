use juniper::GraphQLObject;

use crate::fields::insight_data::{InsightField, UnsecuredLoan};

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(name = "UnsecuredLoansInsights", description = "")]
pub struct Insights {
    pub count: i32,
}

impl Insights {
    pub fn new(insight: Vec<&InsightField<UnsecuredLoan>>) -> Self {
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
    fn it_can_get_unsecured_loans_insights() {
        let query = r#"
        query Insights {
            reports {
              report {
                unsecured_loans {
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
                    "unsecured_loans": {
                      "insights": {
                        "count": 3
                      }
                    }
                  },
                  {
                    "unsecured_loans": {
                      "insights": {
                        "count": 3
                      }
                    }
                  }
                ]
              }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}
