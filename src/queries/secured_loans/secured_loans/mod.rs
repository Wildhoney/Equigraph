use super::secured_loan::SecuredLoanField;
use crate::schema::Context;

pub struct SecuredLoans<'a> {
    pub items: Vec<&'a SecuredLoanField>,
}

#[juniper::graphql_object(context = Context)]
impl SecuredLoans<'_> {
    #[graphql(name = "secured_loan")]
    pub fn secured_loan() -> &Vec<&SecuredLoanField> {
        &self.items
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
            query SecuredLoans {
                secured_loans {
                    secured_loan {
                        account_number
                    }
                }
            }
        "#;

        let expected = graphql_value!({
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
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}
