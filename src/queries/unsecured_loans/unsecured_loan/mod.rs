mod insights;

use self::insights::UnsecuredLoanInsights;
use crate::{
    fields::{
        matched_address::MatchedAddressField,
        payment_history::{PartitionPaymentHistory, PaymentHistoryField},
        AmountField, BalanceField, DateField, FixedPaymentTermsField, PaymentFrequencyField,
    },
    objects::{
        input::Select,
        output::{Company, CompanyClass},
    },
    schema::Context,
    utils::{find_address_by_id, unique_id},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct UnsecuredLoanField {
    #[serde(default = "unique_id")]
    pub id: Uuid,
    #[serde(alias = "accountNumber")]
    pub account_number: String,
    #[serde(alias = "currentBalance")]
    pub current_balance: BalanceField,
    #[serde(alias = "defaultBalance")]
    pub default_balance: BalanceField,
    #[serde(alias = "startBalance")]
    pub start_balance: BalanceField,
    #[serde(alias = "paymentHistory")]
    pub payment_history: Vec<PaymentHistoryField>,
    #[serde(alias = "startDate")]
    pub start_date: DateField,
    #[serde(alias = "lastUpdateDate")]
    pub last_update_date: DateField,
    #[serde(alias = "endDate")]
    pub end_date: Option<DateField>,
    #[serde(alias = "paymentFrequency")]
    pub payment_frequency: PaymentFrequencyField,
    #[serde(alias = "companyName")]
    pub company_name: String,
    #[serde(alias = "companyClass")]
    pub company_class: CompanyClass,
    #[serde(alias = "fixedPaymentTerms")]
    pub fixed_payment_terms: FixedPaymentTermsField,
}

#[juniper::graphql_object(context = Context)]
impl UnsecuredLoanField {
    #[graphql(name = "account_number")]
    pub fn account_number(&self) -> &String {
        &self.account_number
    }

    pub fn company(&self) -> Company {
        Company {
            kind: &self.company_class,
            name: &self.company_name,
        }
    }

    #[graphql(name = "current_balance")]
    pub fn current_balance(&self) -> &AmountField {
        &self.current_balance.balance_amount
    }

    #[graphql(name = "default_balance")]
    pub fn default_balance(&self) -> &AmountField {
        &self.default_balance.balance_amount
    }

    #[graphql(name = "payment_frequency")]
    pub fn payment_frequency(&self) -> &PaymentFrequencyField {
        &self.payment_frequency
    }

    #[graphql(name = "start_balance")]
    pub fn start_balance(&self) -> &AmountField {
        &self.start_balance.balance_amount
    }

    #[graphql(name = "start_date")]
    pub fn start_date(&self) -> &DateField {
        &self.start_date
    }

    #[graphql(name = "last_update_date")]
    pub fn update_date(&self) -> &DateField {
        &self.last_update_date
    }

    #[graphql(name = "end_date")]
    pub fn end_date(&self) -> &Option<DateField> {
        &self.end_date
    }

    #[graphql(name = "fixed_payment_terms")]
    pub fn fixed_payment_terms(&self) -> &FixedPaymentTermsField {
        &self.fixed_payment_terms
    }

    pub fn address(&self, context: &Context) -> Option<&MatchedAddressField> {
        let address = find_address_by_id(self.id, &context.reports)?;
        Some(&address.matched_address)
    }

    pub fn insights(&self) -> UnsecuredLoanInsights {
        UnsecuredLoanInsights::new(self.clone())
    }

    #[graphql(name = "payment_history")]
    pub fn payment_history(&self, select: Select) -> &[PaymentHistoryField] {
        self.payment_history.partition(select)
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_get_unsecured_loan() {
        let query = r#"
        query UnsecuredLoan {
          reports {
            report {
              unsecured_loans {
                unsecured_loan {
                  account_number
                  payment_frequency
                  start_balance {
                    amount
                  }
                  fixed_payment_terms {
                    number_of_payments
                    payment_amount {
                      formatted(zeroes: STRIP)
                    }
                  }
                  start_date {
                    year
                  }
                  end_date {
                    year
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
                      "account_number": "zt6alZd8Gw8CHn/fLohXfBquS9zyU34fw9l2Bn32Jio=",
                      "payment_frequency": "MONTHLY",
                      "start_balance": {
                        "amount": 0
                      },
                      "fixed_payment_terms": {
                        "number_of_payments": 48,
                        "payment_amount": {
                          "formatted": "£210"
                        }
                      },
                      "start_date": {
                        "year": 2020
                      },
                      "end_date": {juniper::Value::Null}
                    },
                    {
                      "account_number": "tSPyQ+mZayou0q6iQiI3680Jg1GD3aCNuzxB8ph2wX8=",
                      "payment_frequency": "MONTHLY",
                      "start_balance": {
                        "amount": 0
                      },
                      "fixed_payment_terms": {
                        "number_of_payments": 60,
                        "payment_amount": {
                          "formatted": "£227"
                        }
                      },
                      "start_date": {
                        "year": 2023
                      },
                      "end_date": {juniper::Value::Null}
                    },
                    {
                      "account_number": "lsrZ9iCAvJoLUh3DsLmI8ynvqpAUObNt/CHTC7WZGrc=",
                      "payment_frequency": "MONTHLY",
                      "start_balance": {
                        "amount": 1920
                      },
                      "fixed_payment_terms": {
                        "number_of_payments": 6,
                        "payment_amount": {
                          "formatted": "£320"
                        }
                      },
                      "start_date": {
                        "year": 2018
                      },
                      "end_date": {
                        "year": 2018
                      }
                    }
                  ]
                }
              },
              {
                "unsecured_loans": {
                  "unsecured_loan": [
                    {
                      "account_number": "zt6alZd8Gw8CHn/fLohXfBquS9zyU34fw9l2Bn32Jio=",
                      "payment_frequency": "MONTHLY",
                      "start_balance": {
                        "amount": 0
                      },
                      "fixed_payment_terms": {
                        "number_of_payments": 48,
                        "payment_amount": {
                          "formatted": "£210"
                        }
                      },
                      "start_date": {
                        "year": 2020
                      },
                      "end_date": {juniper::Value::Null}
                    },
                    {
                      "account_number": "tSPyQ+mZayou0q6iQiI3680Jg1GD3aCNuzxB8ph2wX8=",
                      "payment_frequency": "MONTHLY",
                      "start_balance": {
                        "amount": 0
                      },
                      "fixed_payment_terms": {
                        "number_of_payments": 60,
                        "payment_amount": {
                          "formatted": "£227"
                        }
                      },
                      "start_date": {
                        "year": 2023
                      },
                      "end_date": {juniper::Value::Null}
                    },
                    {
                      "account_number": "lsrZ9iCAvJoLUh3DsLmI8ynvqpAUObNt/CHTC7WZGrc=",
                      "payment_frequency": "MONTHLY",
                      "start_balance": {
                        "amount": 1920
                      },
                      "fixed_payment_terms": {
                        "number_of_payments": 6,
                        "payment_amount": {
                          "formatted": "£320"
                        }
                      },
                      "start_date": {
                        "year": 2018
                      },
                      "end_date": {
                        "year": 2018
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
