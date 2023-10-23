mod insights;

use juniper::FieldResult;

use self::insights::UnsecuredLoansInsights;
use super::unsecured_loan::UnsecuredLoanField;
use crate::{
    fields::insight_data::changes::InsightChanges, objects::input::Since,
    queries::reports::report::ReportField, schema::Context,
};

pub struct UnsecuredLoans<'a> {
    pub report: &'a ReportField,
    pub items: Vec<&'a UnsecuredLoanField>,
}

#[juniper::graphql_object(context = Context)]
impl UnsecuredLoans<'_> {
    #[graphql(name = "unsecured_loan")]
    pub fn unsecured_loan() -> &Vec<&UnsecuredLoanField> {
        &self.items
    }

    pub fn insights() -> UnsecuredLoansInsights {
        UnsecuredLoansInsights::new(self.items.clone())
    }

    pub fn changes(
        &self,
        since: Since,
        context: &Context,
    ) -> FieldResult<Option<InsightChanges<UnsecuredLoanField>>> {
        Ok(InsightChanges::new(
            since,
            self.report,
            &context.reports,
            &|insight_data| &insight_data.unsecured_loan,
        ))
    }
}

#[juniper::graphql_object(context = Context)]
impl InsightChanges<'_, UnsecuredLoanField> {
    pub fn added(&self) -> &Vec<&UnsecuredLoanField> {
        &self.added
    }

    pub fn removed(&self) -> &Vec<&UnsecuredLoanField> {
        &self.removed
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_get_unsecured_loans() {
        let query = r#"
        query UnsecuredLoans {
            reports {
              report {
                unsecured_loans {
                  unsecured_loan {
                    account_number
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
                          "account_number": "zt6alZd8Gw8CHn/fLohXfBquS9zyU34fw9l2Bn32Jio="
                        },
                        {
                          "account_number": "tSPyQ+mZayou0q6iQiI3680Jg1GD3aCNuzxB8ph2wX8="
                        },
                        {
                          "account_number": "lsrZ9iCAvJoLUh3DsLmI8ynvqpAUObNt/CHTC7WZGrc="
                        }
                      ]
                    }
                  },
                  {
                    "unsecured_loans": {
                      "unsecured_loan": [
                        {
                          "account_number": "zt6alZd8Gw8CHn/fLohXfBquS9zyU34fw9l2Bn32Jio="
                        },
                        {
                          "account_number": "tSPyQ+mZayou0q6iQiI3680Jg1GD3aCNuzxB8ph2wX8="
                        },
                        {
                          "account_number": "lsrZ9iCAvJoLUh3DsLmI8ynvqpAUObNt/CHTC7WZGrc="
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
