mod utils;

use self::utils::{get_delta, get_impact, get_polarity};
use super::PaymentHistoryTrait;
use crate::{
    fields::{insight_data::InsightVariant, AmountField},
    objects::{
        input::Since,
        output::{Impact, Polarity},
    },
    parser::types::{Reports, ReportsTrait},
    schema::Context,
};
use uuid::Uuid;

pub struct Changes<'a> {
    pub insight_variant: InsightVariant<'a>,
    pub currency: &'a str,
    pub amount: u32,
    pub compare_with_amount: u32,
}

impl Changes<'_> {
    pub fn new<'a>(
        since: Since,
        id: Uuid,
        amount: &'a AmountField,
        reports: &'a Reports,
    ) -> Option<Changes<'a>> {
        let insight = reports.find_insight_containing_payment_history(id)?;
        let payment_histories = insight.get_payment_history();
        let compare_with_payment_history = payment_histories.since(&since, &id)?;

        let compare_with_amount = compare_with_payment_history
            .account_balance
            .balance_amount
            .amount as u32;

        Some(Changes {
            insight_variant: insight,
            amount: amount.amount as u32,
            compare_with_amount,
            currency: &amount.currency,
        })
    }
}

#[juniper::graphql_object(name = "PaymentHistoryChanges", context = Context)]
impl Changes<'_> {
    pub fn delta(&self) -> AmountField {
        let amount = get_delta(self.amount, self.compare_with_amount);

        AmountField {
            amount,
            currency: self.currency.to_string(),
        }
    }

    pub fn impact(&self) -> Impact {
        get_impact(&self.insight_variant, self.amount, self.compare_with_amount)
    }

    pub fn polarity(&self) -> Polarity {
        get_polarity(&self.insight_variant, self.amount, self.compare_with_amount)
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
          reports {
            report {
              current_accounts {
                current_account {
                  payment_history(select: LATEST) {
                    account_balance {
                      amount
                    }
                    changes(since: PREVIOUS) {
                      delta {
                        amount
                      }
                      impact
                      polarity
                    }
                  }
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
                  "current_account": [
                    {
                      "payment_history": [
                        {
                          "account_balance": {
                            "amount": 0
                          },
                          "changes": {
                            "delta": {
                              "amount": 0
                            },
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
                            "delta": {
                              "amount": 0
                            },
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
                            "delta": {
                              "amount": 0
                            },
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
                            "delta": {
                              "amount": 0
                            },
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
                            "delta": {
                              "amount": {-2}
                            },
                            "impact": "LOW",
                            "polarity": "NEGATIVE"
                          }
                        }
                      ]
                    }
                  ]
                }
              },
              {
                "current_accounts": {
                  "current_account": [
                    {
                      "payment_history": [
                        {
                          "account_balance": {
                            "amount": 0
                          },
                          "changes": {
                            "delta": {
                              "amount": 0
                            },
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
                            "delta": {
                              "amount": 0
                            },
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
                            "delta": {
                              "amount": 0
                            },
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
                            "delta": {
                              "amount": {-2}
                            },
                            "impact": "LOW",
                            "polarity": "NEGATIVE"
                          }
                        }
                      ]
                    }
                  ]
                }
              }
            ]
          }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}
