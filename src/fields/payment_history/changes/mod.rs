mod utils;

use self::utils::{get_delta, get_impact, get_polarity};
use super::utils::get_payment_history_by_id;
use crate::{
    fields::insight_data::InsightKind,
    objects::{
        input::Since,
        output::{Impact, Polarity},
    },
    parser::types::Reports,
    schema::Context,
};
use uuid::Uuid;

pub struct PaymentHistoryChanges<'a> {
    pub kind: InsightKind<'a>,
    pub amount: u32,
    pub compare_with_amount: u32,
}

impl PaymentHistoryChanges<'_> {
    pub fn new(
        since: Since,
        id: Uuid,
        amount: u32,
        reports: &Reports,
    ) -> Option<PaymentHistoryChanges> {
        let payment_histories = get_payment_history_by_id(id, reports)?;
        let current_index = payment_histories
            .list
            .iter()
            .position(|payment_history| payment_history.id == id)?;

        let compare_with_payment_history = match since {
            Since::Previous => payment_histories.list.get(current_index + 1),
            Since::Next => {
                (current_index != 0).then(|| payment_histories.list.get(current_index - 1))?
            }
            Since::First => payment_histories.list.first(),
            Since::Last => payment_histories.list.last(),
        }?;

        let compare_with_amount = compare_with_payment_history
            .account_balance
            .balance_amount
            .amount as u32;

        Some(PaymentHistoryChanges {
            kind: payment_histories.insight,
            amount,
            compare_with_amount,
        })
    }
}

#[juniper::graphql_object(context = Context)]
impl PaymentHistoryChanges<'_> {
    pub fn delta(&self) -> i32 {
        get_delta(self.amount, self.compare_with_amount)
    }

    pub fn impact(&self) -> Impact {
        get_impact(&self.kind, self.amount, self.compare_with_amount)
    }

    pub fn polarity(&self) -> Polarity {
        get_polarity(&self.kind, self.amount, self.compare_with_amount)
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_display_payment_history_changes() {
        let query = r#"
            query PaymentHistory {
                current_accounts {
                  current_account {
                    payment_history(select: LATEST) {
                      account_balance {
                        amount
                      }
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

        let expected = graphql_value!({
          "current_accounts": {
            "current_account": [
              {
                "payment_history": [
                  {
                    "account_balance": {
                      "amount": 0
                    },
                    "changes": {
                      "delta": 0,
                      "impact": "NONE",
                      "polarity": "UNCHANGED"
                    }
                  }
                ]
              },
              {
                "payment_history": [
                  {
                    "account_balance": {
                      "amount": 0
                    },
                    "changes": {
                      "delta": 0,
                      "impact": "NONE",
                      "polarity": "UNCHANGED"
                    }
                  }
                ]
              },
              {
                "payment_history": [
                  {
                    "account_balance": {
                      "amount": 0
                    },
                    "changes": {
                      "delta": 0,
                      "impact": "NONE",
                      "polarity": "UNCHANGED"
                    }
                  }
                ]
              },
              {
                "payment_history": [
                  {
                    "account_balance": {
                      "amount": 0
                    },
                    "changes": {
                      "delta": 0,
                      "impact": "NONE",
                      "polarity": "UNCHANGED"
                    }
                  }
                ]
              },
              {
                "payment_history": [
                  {
                    "account_balance": {
                      "amount": 0
                    },
                    "changes": {
                      "delta": {-2},
                      "impact": "LOW",
                      "polarity": "NEGATIVE"
                    }
                  }
                ]
              }
            ]
          }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}
