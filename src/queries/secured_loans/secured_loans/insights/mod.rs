use crate::queries::secured_loans::secured_loan::SecuredLoanField;
use juniper::GraphQLObject;

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(description = "")]
pub struct SecuredLoansInsights {
    pub count: i32,
}

impl SecuredLoansInsights {
    pub fn new(insight: Vec<&SecuredLoanField>) -> Self {
        SecuredLoansInsights {
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
        query SecuredLoansInsights {
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
