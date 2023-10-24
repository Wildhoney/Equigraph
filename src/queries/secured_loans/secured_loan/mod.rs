mod insights;

use self::insights::SecuredLoanInsights;
use crate::{
    fields::{
        matched_address::MatchedAddressField,
        payment_history::{PartitionPaymentHistory, PaymentHistoryField},
        AmountField, BalanceField, DateField, FixedPaymentTermsField, LoanTypeField,
        PaymentFrequencyField,
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
pub struct SecuredLoanField {
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
    #[serde(alias = "loanType")]
    pub loan_type: LoanTypeField,
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
    pub flexible: bool,
    #[serde(alias = "fixedPaymentTerms")]
    pub fixed_payment_terms: FixedPaymentTermsField,
}

#[juniper::graphql_object(context = Context)]
impl SecuredLoanField {
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

    #[graphql(name = "loan_type")]
    pub fn loan_type(&self) -> &LoanTypeField {
        &self.loan_type
    }

    #[graphql(name = "is_flexible")]
    pub fn is_flexible(&self) -> bool {
        self.flexible
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

    pub fn insights(&self) -> SecuredLoanInsights {
        SecuredLoanInsights::new(self.clone())
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
    fn it_can_get_secured_loan() {
        let query = r#"
        query SecuredLoan {
            reports {
              report {
                secured_loans {
                  secured_loan {
                    account_number
                    payment_frequency
                    is_flexible
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
                    "secured_loans": {
                      "secured_loan": [
                        {
                          "account_number": "kHbepkF0tHD7+oaFLYE/+XMUAuTp58af5EZrYeBtVjs=",
                          "payment_frequency": "MONTHLY",
                          "is_flexible": false,
                          "start_balance": {
                            "amount": 0
                          },
                          "fixed_payment_terms": {
                            "number_of_payments": 300,
                            "payment_amount": {
                              "formatted": "£2,282"
                            }
                          },
                          "start_date": {
                            "year": 2022
                          },
                          "end_date": {juniper::Value::Null}
                        },
                        {
                          "account_number": "r9jjexGpGIiqxQJx1AODd+N2KFtABRCSglQNZ26UguE=",
                          "payment_frequency": "MONTHLY",
                          "is_flexible": false,
                          "start_balance": {
                            "amount": 0
                          },
                          "fixed_payment_terms": {
                            "number_of_payments": 0,
                            "payment_amount": {
                              "formatted": "£1,641"
                            }
                          },
                          "start_date": {
                            "year": 2017
                          },
                          "end_date": {
                            "year": 2022
                          }
                        }
                      ]
                    }
                  },
                  {
                    "secured_loans": {
                      "secured_loan": [
                        {
                          "account_number": "kHbepkF0tHD7+oaFLYE/+XMUAuTp58af5EZrYeBtVjs=",
                          "payment_frequency": "MONTHLY",
                          "is_flexible": false,
                          "start_balance": {
                            "amount": 0
                          },
                          "fixed_payment_terms": {
                            "number_of_payments": 300,
                            "payment_amount": {
                              "formatted": "£2,282"
                            }
                          },
                          "start_date": {
                            "year": 2022
                          },
                          "end_date": {juniper::Value::Null}
                        },
                        {
                          "account_number": "r9jjexGpGIiqxQJx1AODd+N2KFtABRCSglQNZ26UguE=",
                          "payment_frequency": "MONTHLY",
                          "is_flexible": false,
                          "start_balance": {
                            "amount": 0
                          },
                          "fixed_payment_terms": {
                            "number_of_payments": 0,
                            "payment_amount": {
                              "formatted": "£1,641"
                            }
                          },
                          "start_date": {
                            "year": 2017
                          },
                          "end_date": {
                            "year": 2022
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
