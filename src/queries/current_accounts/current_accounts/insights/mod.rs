use crate::fields::insight_data::{CurrentAccount, InsightField};
use juniper::GraphQLObject;

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(name = "CurrentAccountsInsights", description = "")]
pub struct Insights {
    pub count: i32,
}

impl Insights {
    pub fn new(items: &Vec<&InsightField<CurrentAccount>>) -> Self {
        Insights {
            count: items.len() as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_get_current_accounts_insights() {
        let query = r#"
        query CurrentAccountsInsights {
            reports {
              report {
                current_accounts {
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
                    "current_accounts": {
                      "insights": {
                        "count": 5
                      }
                    }
                  },
                  {
                    "current_accounts": {
                      "insights": {
                        "count": 4
                      }
                    }
                  }
                ]
              }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}
