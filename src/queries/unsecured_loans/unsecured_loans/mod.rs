mod insights;

use self::insights::Insights;
use crate::{
    fields::insight_data::{changes::Changes, InsightField, UnsecuredLoan},
    objects::input::Since,
    queries::reports::report::ReportField,
    schema::Context,
};
use juniper::FieldResult;

pub struct UnsecuredLoans<'a> {
    pub report: &'a ReportField,
    pub items: Vec<&'a InsightField<UnsecuredLoan>>,
}

#[juniper::graphql_object(context = Context)]
impl UnsecuredLoans<'_> {
    #[graphql(name = "unsecured_loan")]
    pub fn unsecured_loan() -> &Vec<&InsightField<UnsecuredLoan>> {
        &self.items
    }

    pub fn insights() -> Insights {
        Insights::new(self.items.clone())
    }

    pub fn changes(
        &self,
        since: Since,
        context: &Context,
    ) -> FieldResult<Option<Changes<InsightField<UnsecuredLoan>>>> {
        Ok(Changes::new(
            since,
            self.report,
            &context.reports,
            &|insight_data| &insight_data.unsecured_loan,
        ))
    }
}

#[juniper::graphql_object(context = Context)]
impl Changes<'_, InsightField<UnsecuredLoan>> {
    pub fn added(&self) -> &Vec<&InsightField<UnsecuredLoan>> {
        &self.added
    }

    pub fn removed(&self) -> &Vec<&InsightField<UnsecuredLoan>> {
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
