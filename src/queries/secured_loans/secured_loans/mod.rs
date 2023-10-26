mod insights;

use self::insights::Insights;
use crate::{
    fields::insight_data::{changes::Changes, InsightField, SecuredLoan},
    objects::input::Since,
    queries::reports::report::ReportField,
    schema::Context,
};
use juniper::FieldResult;

pub struct SecuredLoans<'a> {
    pub report: &'a ReportField,
    pub items: Vec<&'a InsightField<SecuredLoan>>,
}

#[juniper::graphql_object(context = Context)]
impl SecuredLoans<'_> {
    #[graphql(name = "secured_loan")]
    pub fn secured_loan() -> &Vec<&InsightField<SecuredLoan>> {
        &self.items
    }

    pub fn insights() -> Insights {
        Insights::new(self.items.clone())
    }

    pub fn changes(
        &self,
        since: Since,
        context: &Context,
    ) -> FieldResult<Option<Changes<InsightField<SecuredLoan>>>> {
        Ok(Changes::new(
            since,
            self.report,
            &context.reports,
            &|insight_data| &insight_data.secured_loan,
        ))
    }
}

#[juniper::graphql_object(context = Context)]
impl Changes<'_, InsightField<SecuredLoan>> {
    pub fn added(&self) -> &Vec<&InsightField<SecuredLoan>> {
        &self.added
    }

    pub fn removed(&self) -> &Vec<&InsightField<SecuredLoan>> {
        &self.removed
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_get_secured_loans() {
        let query = r#"
        query SecuredLoans {
            reports {
              report {
                secured_loans {
                  secured_loan {
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
                    "secured_loans": {
                      "secured_loan": [
                        {
                          "account_number": "kHbepkF0tHD7+oaFLYE/+XMUAuTp58af5EZrYeBtVjs="
                        },
                        {
                          "account_number": "r9jjexGpGIiqxQJx1AODd+N2KFtABRCSglQNZ26UguE="
                        }
                      ]
                    }
                  },
                  {
                    "secured_loans": {
                      "secured_loan": [
                        {
                          "account_number": "kHbepkF0tHD7+oaFLYE/+XMUAuTp58af5EZrYeBtVjs="
                        },
                        {
                          "account_number": "r9jjexGpGIiqxQJx1AODd+N2KFtABRCSglQNZ26UguE="
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
