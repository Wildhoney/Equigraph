mod insights;

use self::insights::UnsecuredLoansInsights;
use super::unsecured_loan::UnsecuredLoanField;
use crate::schema::Context;

pub struct UnsecuredLoans<'a> {
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
                unsecured_loans {
                    unsecured_loan {
                        account_number
                    }
                }
            }
        "#;

        let expected = graphql_value!({
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
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}