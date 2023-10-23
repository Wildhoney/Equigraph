mod insights;

use self::insights::CurrentAccountsInsights;
use super::current_account::CurrentAccountField;
use crate::{
    fields::insight_data::changes::InsightChanges, objects::input::Since,
    queries::reports::report::ReportField, schema::Context,
};
use juniper::FieldResult;

pub struct CurrentAccounts<'a> {
    pub report: &'a ReportField,
    pub items: Vec<&'a CurrentAccountField>,
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccounts<'_> {
    #[graphql(name = "current_account")]
    pub fn current_account() -> &Vec<&CurrentAccountField> {
        &self.items
    }

    pub fn insights() -> FieldResult<CurrentAccountsInsights> {
        Ok(CurrentAccountsInsights::new(&self.items))
    }

    pub fn changes(
        &self,
        since: Since,
        context: &Context,
    ) -> FieldResult<Option<InsightChanges<CurrentAccountField>>> {
        Ok(InsightChanges::new(
            since,
            self.report,
            &context.reports,
            &|insight_data| &insight_data.current_account,
        ))
    }
}

#[juniper::graphql_object(context = Context)]
impl InsightChanges<'_, CurrentAccountField> {
    pub fn added(&self) -> &Vec<&CurrentAccountField> {
        &self.added
    }

    pub fn removed(&self) -> &Vec<&CurrentAccountField> {
        &self.removed
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
        query CurrentAccounts {
            reports {
              report {
                current_accounts {
                  current_account {
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
                  },
                  {
                    "current_accounts": {
                      "current_account": [
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
                  }
                ]
              }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}
