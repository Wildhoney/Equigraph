use crate::{
    fields::{
        insight_data::{InsightField, SecuredLoan},
        DateField,
    },
    objects::output::PaymentAnalysis,
    schema::Context,
    utils::compute_scheduled_end_date,
};

#[derive(Debug, PartialEq)]
pub struct Insights<'a> {
    pub insight: &'a InsightField<SecuredLoan>,
}

impl Insights<'_> {
    pub fn new<'a>(insight: &'a InsightField<SecuredLoan>) -> Insights<'a> {
        Insights { insight }
    }
}

#[juniper::graphql_object(name = "SecuredLoanInsights", context = Context)]
impl Insights<'_> {
    pub fn active(&self) -> bool {
        self.insight.end_date.is_none()
    }

    #[graphql(name = "current_end_date")]
    pub fn current_end_date(&self) -> Option<DateField> {
        compute_scheduled_end_date(
            self.insight
                .fixed_payment_terms
                .as_ref()
                .unwrap()
                .number_of_payments,
            &self.insight.payment_frequency,
        )
    }

    #[graphql(name = "payment_analysis")]
    pub fn payment_analysis(&self) -> PaymentAnalysis {
        PaymentAnalysis {
            active: self.insight.end_date.is_none(),
            total: self
                .insight
                .fixed_payment_terms
                .as_ref()
                .unwrap()
                .number_of_payments,
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
    fn it_can_get_secured_loan_insights() {
        let query = r#"
        query Insights {
            reports {
              report {
                secured_loans {
                  secured_loan {
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
                    "secured_loans": {
                      "secured_loan": [
                        {
                          "insights": {
                            "active": true,
                            "payment_analysis": {
                              "made": 15,
                              "total": 300,
                              "remaining": 285
                            }
                          }
                        },
                        {
                          "insights": {
                            "active": false,
                            "payment_analysis": {
                              "made": 48,
                              "total": {juniper::Value::Null},
                              "remaining": {juniper::Value::Null}
                            }
                          }
                        }
                      ]
                    }
                  },
                  {
                    "secured_loans": {
                      "secured_loan": [
                        {
                          "insights": {
                            "active": true,
                            "payment_analysis": {
                              "made": 15,
                              "total": 300,
                              "remaining": 285
                            }
                          }
                        },
                        {
                          "insights": {
                            "active": false,
                            "payment_analysis": {
                              "made": 48,
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
