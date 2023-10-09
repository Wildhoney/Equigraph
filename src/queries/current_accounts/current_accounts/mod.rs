use juniper::FieldResult;

use super::{current_account::CurrentAccountField, insights::CurrentAccountsInsights};
use crate::schema::Context;

pub struct CurrentAccounts {
    pub items: Vec<CurrentAccountField>,
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccounts {
    #[graphql(name = "current_account")]
    pub fn current_account() -> &Vec<CurrentAccountField> {
        &self.items
    }

    pub fn insights() -> FieldResult<CurrentAccountsInsights> {
        Ok(CurrentAccountsInsights::new(&self.items))
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
            query CurrentAccount {
                current_accounts {
                    current_account {
                      account_number
                    }
                }
            }
        "#;

        let expected = graphql_value!({
            "current_accounts": {
                "current_account": [
                  {
                    "account_number": "zGML/Ld93it5j86rAFo2wxM8oGHNdoWJj4WTwoRmkcc="
                  },
                  {
                    "account_number": "3oEmu6B1FCnWguuTc93gXTPtT3NMcaxCKSm2MLOFvMw="
                  },
                  {
                    "account_number": "5YduNv4WxF4SOS0GqS8uh/yOA/TFTgsQT1uH5kAB8RQ="
                  },
                  {
                    "account_number": "iMt7bI9kNQtpjsWYsMr69lgUsgyg5XQVMF4dhBknm3E="
                  },
                  {
                    "account_number": "LFTlUsWtDduQAo2L7zJOtNKDI86DztlNPL6Fg7iz4+M="
                  }
                ]
              }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}