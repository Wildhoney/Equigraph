use crate::queries::unsecured_loans::unsecured_loan::UnsecuredLoanField;
use juniper::GraphQLObject;

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(description = "")]
pub struct UnsecuredLoansInsights {
    pub count: i32,
}

impl UnsecuredLoansInsights {
    pub fn new(insight: Vec<&UnsecuredLoanField>) -> Self {
        UnsecuredLoansInsights {
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
        query UnsecuredLoansInsights {
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
