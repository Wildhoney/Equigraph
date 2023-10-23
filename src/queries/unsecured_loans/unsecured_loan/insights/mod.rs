use super::UnsecuredLoanField;
use crate::{
    fields::DateField, objects::output::PaymentAnalysis, schema::Context,
    utils::compute_scheduled_end_date,
};

#[derive(Debug, PartialEq)]
pub struct UnsecuredLoanInsights {
    pub insight: UnsecuredLoanField,
}

impl UnsecuredLoanInsights {
    pub fn new(insight: UnsecuredLoanField) -> Self {
        UnsecuredLoanInsights { insight }
    }
}

#[juniper::graphql_object(context = Context)]
impl UnsecuredLoanInsights {
    pub fn active(&self) -> bool {
        self.insight.end_date.is_none()
    }

    #[graphql(name = "current_end_date")]
    pub fn current_end_date(&self) -> Option<DateField> {
        compute_scheduled_end_date(
            self.insight.fixed_payment_terms.number_of_payments,
            &self.insight.payment_frequency,
        )
    }

    #[graphql(name = "payment_analysis")]
    pub fn payment_analysis(&self) -> PaymentAnalysis {
        PaymentAnalysis {
            active: self.insight.end_date.is_none(),
            total: self.insight.fixed_payment_terms.number_of_payments,
            payments: self.insight.payment_history.len() as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_get_unsecured_loan_insights() {
        let query = r#"
        query UnsecuredLoanInsights {
          reports {
            report {
              unsecured_loans {
                unsecured_loan {
                  insights {
                    active
                    payment_analysis {
                      made
                      total
                      remaining
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
                "unsecured_loans": {
                  "unsecured_loan": [
                    {
                      "insights": {
                        "active": true,
                        "payment_analysis": {
                          "made": 42,
                          "total": 48,
                          "remaining": 6
                        }
                      }
                    },
                    {
                      "insights": {
                        "active": true,
                        "payment_analysis": {
                          "made": 5,
                          "total": 60,
                          "remaining": 55
                        }
                      }
                    },
                    {
                      "insights": {
                        "active": false,
                        "payment_analysis": {
                          "made": 7,
                          "total": {juniper::Value::Null},
                          "remaining": {juniper::Value::Null}
                        }
                      }
                    }
                  ]
                }
              },
              {
                "unsecured_loans": {
                  "unsecured_loan": [
                    {
                      "insights": {
                        "active": true,
                        "payment_analysis": {
                          "made": 42,
                          "total": 48,
                          "remaining": 6
                        }
                      }
                    },
                    {
                      "insights": {
                        "active": true,
                        "payment_analysis": {
                          "made": 5,
                          "total": 60,
                          "remaining": 55
                        }
                      }
                    },
                    {
                      "insights": {
                        "active": false,
                        "payment_analysis": {
                          "made": 7,
                          "total": {juniper::Value::Null},
                          "remaining": {juniper::Value::Null}
                        }
                      }
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
