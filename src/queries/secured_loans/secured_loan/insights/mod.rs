use super::SecuredLoanField;
use crate::{
    fields::DateField, objects::output::PaymentAnalysis, schema::Context,
    utils::compute_scheduled_end_date,
};

#[derive(Debug, PartialEq)]
pub struct SecuredLoanInsights {
    pub insight: SecuredLoanField,
}

impl SecuredLoanInsights {
    pub fn new(insight: SecuredLoanField) -> Self {
        SecuredLoanInsights { insight }
    }
}

#[juniper::graphql_object(context = Context)]
impl SecuredLoanInsights {
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
    fn it_can_get_current_accounts() {
        let query = r#"
            query SecuredLoanInsights {
                secured_loans {
                    secured_loan {
                        insights {
                        active
                        current_end_date {
                            formatted(like:"%d/%m/%Y")
                        }
                        payment_analysis {
                            made
                            total
                            remaining
                        }
                        }
                    }
                }
            }
        "#;

        let expected = graphql_value!({
            "secured_loans": {
                "secured_loan": [
                  {
                    "insights": {
                      "active": true,
                      "current_end_date": {
                        "formatted": "13/10/2048"
                      },
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
                      "current_end_date": {juniper::Value::Null},
                      "payment_analysis": {
                        "made": 48,
                        "total": {juniper::Value::Null},
                        "remaining": {juniper::Value::Null}
                      }
                    }
                  }
                ]
              }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}