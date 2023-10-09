use juniper::GraphQLObject;

use super::current_account::CurrentAccountField;

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(description = "")]
pub struct CurrentAccountsInsights {
    pub count: i32,
}

impl CurrentAccountsInsights {
    pub fn new(items: &Vec<CurrentAccountField>) -> Self {
        CurrentAccountsInsights {
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
    fn it_can_get_current_accounts() {
        let query = r#"
            query CurrentAccount {
                current_accounts {
                    insights {
                        count
                    }
                }
            }
        "#;

        let expected = graphql_value!({
            "current_accounts": {
                "insights": {
                  "count": 5
                }
              }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}
