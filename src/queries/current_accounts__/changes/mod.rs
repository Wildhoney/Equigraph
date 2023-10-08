mod utils;

use self::utils::{get_delta, get_impact, get_polarity};
use super::fields::{CurrentAccountField, PaymentHistoryField};
use crate::{
    objects::{
        input::Since,
        output::{Impact, Polarity},
    },
    schema::Context,
};

#[derive(Debug, PartialEq)]
pub struct CurrentAccountChanges<'a> {
    pub since: Since,
    pub account: &'a CurrentAccountField,
    pub payment_history: &'a PaymentHistoryField,
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountChanges<'_> {
    pub fn delta(&self) -> Option<i32> {
        get_delta(&self.since, &self.account, &self.payment_history)
    }

    pub fn impact(&self) -> Option<Impact> {
        get_impact(&self.since, &self.account, &self.payment_history)
    }

    pub fn polarity(&self) -> Option<Polarity> {
        get_polarity(&self.since, &self.account, &self.payment_history)
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_get_changes() {
        let query = r#"
        query Changes {
            current_accounts {
                current_account {
                    payment_history(select: LATEST) {
                        changes(since: PREVIOUS) {
                            delta
                            impact
                            polarity
                        }
                    }
                }
            }
        }
    "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "current_accounts": {
                    "current_account": [
                        { "payment_history": [{ "changes": { "delta": 0, "impact": "NONE", "polarity": "UNCHANGED" } }] },
                        { "payment_history": [{ "changes": { "delta": 0, "impact": "NONE", "polarity": "UNCHANGED" } }] },
                        { "payment_history": [{ "changes": { "delta": 0, "impact": "NONE", "polarity": "UNCHANGED" } }] },
                        { "payment_history": [{ "changes": { "delta": 0, "impact": "NONE", "polarity": "UNCHANGED" } }] },
                        { "payment_history": [{ "changes": { "delta": (-2), "impact": "LOW", "polarity": "POSITIVE" } }] },
                    ]
                },
            })
        );
    }
}
