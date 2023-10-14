mod utils;

use super::{AmountField, BalanceField, PaymentStatusField};
use crate::{
    objects::{
        input::Since,
        output::{Impact, Polarity},
    },
    schema::Context,
    utils::unique_id,
};
use juniper::GraphQLObject;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct PaymentHistoryField {
    #[serde(default = "unique_id")]
    pub id: Uuid,
    #[serde(alias = "accountBalance")]
    pub account_balance: BalanceField,
    #[serde(alias = "ageInMonths")]
    pub age_in_months: i32,
    #[serde(alias = "paymentStatus")]
    pub payment_status: PaymentStatusField,
}

#[juniper::graphql_object(context = Context)]
impl PaymentHistoryField {
    #[graphql(name = "age_in_months")]
    pub fn age_in_months(&self) -> i32 {
        self.age_in_months
    }

    #[graphql(name = "payment_status")]
    pub fn payment_status(&self) -> &PaymentStatusField {
        &self.payment_status
    }

    #[graphql(name = "account_balance")]
    pub fn account_balance(&self) -> &AmountField {
        &self.account_balance.balance_amount
    }

    pub fn changes(&self, context: &Context, since: Since) -> Option<Changes> {
        let payment_histories = utils::get_payment_histories_by_id(self.id, &context.reports)?;
        let current_index = payment_histories
            .iter()
            .position(|payment_history| payment_history.id == self.id)?;

        let compare_with_payment_history = match since {
            Since::Previous => payment_histories.get(current_index + 1),
            Since::Next => {
                (current_index != 0).then(|| payment_histories.get(current_index - 1))?
            }
            Since::First => payment_histories.first(),
            Since::Last => payment_histories.last(),
        }?;

        let amount = self.account_balance.balance_amount.amount as u32;
        let compare_with_amount = compare_with_payment_history
            .account_balance
            .balance_amount
            .amount as u32;

        Some(Changes {
            delta: get_delta(amount, compare_with_amount),
            impact: get_impact(amount, compare_with_amount),
            polarity: get_polarity(amount, compare_with_amount),
        })
    }
}

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(description = "")]
struct Changes {
    delta: i32,
    impact: Impact,
    polarity: Polarity,
}

pub fn get_delta(lhs_score: u32, rhs_score: u32) -> i32 {
    (lhs_score as i32 - rhs_score as i32) as i32
}

pub fn get_polarity(lhs_score: u32, rhs_score: u32) -> Polarity {
    match get_delta(lhs_score, rhs_score) {
        delta if delta > 0 => Polarity::Positive,
        delta if delta < 0 => Polarity::Negative,
        _ => Polarity::Unchanged,
    }
}

pub fn get_impact(lhs_score: u32, rhs_score: u32) -> Impact {
    match get_delta(lhs_score, rhs_score) {
        delta if delta == 0 => Impact::None,
        delta if delta < 200 => Impact::Low,
        delta if delta >= 200 => Impact::High,
        _ => Impact::None,
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
