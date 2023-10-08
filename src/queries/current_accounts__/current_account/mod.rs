use super::payment_history::CurrentAccountPaymentHistory;
use crate::{
    fields::matched_address::MatchedAddressField,
    insights::{get_current_account_insights, CurrentAccountInsight},
    objects::{
        input::{Format, Select},
        output::{Balance, Company, Date},
    },
    parser::{fields::PaymentFrequencyField, types::Report},
    schema::Context,
    utils::get_date,
};
use juniper::FieldResult;

impl CurrentAccountInsight<'_> {
    pub fn new<'a>(report: Option<&'a Report>) -> Vec<CurrentAccountInsight> {
        match report {
            Some(report) => get_current_account_insights(report),
            None => vec![],
        }
    }
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountInsight<'_> {
    #[graphql(name = "account_number")]
    pub fn account_number(&self) -> &String {
        &self.current_account.account_number
    }

    pub fn company(&self) -> Company {
        Company {
            kind: &self.current_account.company_class,
            name: &self.current_account.company_name,
        }
    }

    #[graphql(name = "has_overdraft")]
    pub fn has_overdraft(&self) -> &bool {
        &self.current_account.overdraft
    }

    #[graphql(name = "current_balance")]
    pub fn current_balance(&self) -> Balance {
        Balance {
            amount: self.current_account.current_balance.balance_amount.amount,
            currency: &self.current_account.current_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "default_balance")]
    pub fn default_balance(&self) -> Balance {
        Balance {
            amount: self.current_account.default_balance.balance_amount.amount,
            currency: &self.current_account.default_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "start_balance")]
    pub fn start_balance(&self) -> Balance {
        Balance {
            amount: self.current_account.start_balance.balance_amount.amount,
            currency: &self.current_account.start_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "credit_limit")]
    pub fn credit_limit(&self) -> Option<Balance> {
        match self.current_account.credit_limit {
            Some(ref credit_limit) => Some(Balance {
                amount: credit_limit.limit.amount,
                currency: &credit_limit.limit.currency,
            }),
            None => None,
        }
    }

    #[graphql(name = "payment_frequency")]
    pub fn payment_frequency(&self) -> &PaymentFrequencyField {
        &self.current_account.payment_frequency
    }

    #[graphql(name = "start_date")]
    pub fn start_date(&self, format: Format) -> Option<Date> {
        get_date(
            self.current_account.start_date.year,
            self.current_account.start_date.month,
            self.current_account.start_date.day,
            format,
        )
    }

    #[graphql(name = "payment_history")]
    pub fn payment_history(&self, select: Option<Select>) -> Vec<CurrentAccountPaymentHistory> {
        CurrentAccountPaymentHistory::new(select, self.current_account)
    }

    pub fn address(&self) -> FieldResult<&MatchedAddressField> {
        Ok(self.address)
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_get_current_account() {
        let query = r#"
            query CurrentAccount {
                current_accounts {
                    current_account {
                        account_number
                    }
                }
            }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "current_accounts": {
                    "current_account": [
                        { "account_number": "zGML/Ld93it5j86rAFo2wxM8oGHNdoWJj4WTwoRmkcc=" },
                        { "account_number": "3oEmu6B1FCnWguuTc93gXTPtT3NMcaxCKSm2MLOFvMw=" },
                        { "account_number": "5YduNv4WxF4SOS0GqS8uh/yOA/TFTgsQT1uH5kAB8RQ=" },
                        { "account_number": "iMt7bI9kNQtpjsWYsMr69lgUsgyg5XQVMF4dhBknm3E=" },
                        { "account_number": "LFTlUsWtDduQAo2L7zJOtNKDI86DztlNPL6Fg7iz4+M=" },
                    ]
                },
            })
        );
    }

    #[test]
    fn it_can_get_current_account_with_start_date() {
        let query = r#"
            query CurrentAccount {
                current_accounts {
                    current_account {
                        start_date(format: "%Y-%m-%d")
                    }
                }
            }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "current_accounts": {
                    "current_account": [
                        { "start_date": "2004-11-10" },
                        { "start_date": "2013-06-28" },
                        { "start_date": "2016-06-06" },
                        { "start_date": "2019-09-29" },
                        { "start_date": "2019-07-17" },
                    ]
                },
            })
        );
    }
}
