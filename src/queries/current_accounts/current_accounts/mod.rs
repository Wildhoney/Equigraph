mod insights;

use self::insights::Insights;
use crate::{
    fields::insight_data::{changes::Changes, CurrentAccount, InsightField},
    objects::input::Since,
    parser::types::{Report, ReportTrait, ReportsTrait},
    schema::Context,
};
use juniper::FieldResult;

pub struct CurrentAccounts<'a> {
    pub report: &'a Report,
    pub items: Vec<&'a InsightField<CurrentAccount>>,
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccounts<'_> {
    #[graphql(name = "current_account")]
    pub fn current_account() -> &Vec<&InsightField<CurrentAccount>> {
        &self.items
    }

    pub fn insights() -> FieldResult<Insights> {
        Ok(Insights::new(&self.items))
    }

    pub fn changes(
        &self,
        since: Since,
        context: &Context,
    ) -> Option<Changes<&InsightField<CurrentAccount>>> {
        let current_accounts = self.report.get_current_accounts();

        let compare_with_current_accounts = context
            .reports
            .since(&since, &self.report.id)?
            .get_current_accounts();

        Some(Changes::new(
            current_accounts,
            compare_with_current_accounts,
        ))
    }
}

#[juniper::graphql_object(context = Context)]
impl Changes<&InsightField<CurrentAccount>> {
    pub fn added(&self) -> &Vec<&InsightField<CurrentAccount>> {
        &self.added
    }

    pub fn removed(&self) -> &Vec<&InsightField<CurrentAccount>> {
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
