use crate::{
    insights::{get_current_account_insights, CurrentAccountInsight},
    parser::types::Report,
    schema::Context,
};

#[derive(Debug, PartialEq)]
pub struct CurrentAccountInsights<'a> {
    pub current_accounts: Vec<CurrentAccountInsight<'a>>,
}

impl CurrentAccountInsights<'_> {
    pub fn new<'a>(report: Option<&'a Report>) -> Option<CurrentAccountInsights> {
        match report {
            Some(report) => Some(CurrentAccountInsights {
                current_accounts: get_current_account_insights(report),
            }),
            None => None,
        }
    }
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountInsights<'_> {
    #[graphql(name = "count")]
    pub fn count(&self) -> i32 {
        self.current_accounts.len() as i32
    }

    #[graphql(name = "has_overdraft")]
    pub fn has_overdraft(&self) -> bool {
        self.current_accounts
            .iter()
            .any(|x| x.current_account.overdraft)
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
        query Insights {
            current_accounts {
                insights {
                    count
                    has_overdraft
                }
            }
        }
    "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "current_accounts": {"insights": { "count": 5, "has_overdraft": false }}
            })
        );
    }
}
