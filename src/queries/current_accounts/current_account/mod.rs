use super::{
    fields::CurrentAccountField, payment_history::CurrentAccountPaymentHistory, utils::get_accounts,
};
use crate::{
    objects::{
        input::Select,
        output::{Balance, Company, Date},
    },
    parser::{fields::PaymentFrequencyField, types::Report},
    schema::Context,
};

#[derive(Debug, PartialEq)]
pub struct CurrentAccount<'a> {
    pub account: &'a CurrentAccountField,
}

impl CurrentAccount<'_> {
    pub fn new<'a>(report: Option<&'a Report>) -> Vec<CurrentAccount> {
        match report {
            Some(report) => get_accounts(report)
                .iter()
                .map(|current_account| CurrentAccount {
                    account: &current_account,
                })
                .collect::<Vec<_>>(),
            None => vec![],
        }
    }
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccount<'_> {
    #[graphql(name = "account_number")]
    pub fn account_number(&self) -> &String {
        &self.account.account_number
    }

    pub fn company(&self) -> Company {
        Company {
            kind: &self.account.company_class,
            name: &self.account.company_name,
        }
    }

    #[graphql(name = "has_overdraft")]
    pub fn has_overdraft(&self) -> &bool {
        &self.account.overdraft
    }

    #[graphql(name = "current_balance")]
    pub fn current_balance(&self) -> Balance {
        Balance {
            amount: self.account.current_balance.balance_amount.amount,
            currency: &self.account.current_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "default_balance")]
    pub fn default_balance(&self) -> Balance {
        Balance {
            amount: self.account.default_balance.balance_amount.amount,
            currency: &self.account.default_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "start_balance")]
    pub fn start_balance(&self) -> Balance {
        Balance {
            amount: self.account.start_balance.balance_amount.amount,
            currency: &self.account.start_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "payment_frequency")]
    pub fn payment_frequency(&self) -> &PaymentFrequencyField {
        &self.account.payment_frequency
    }

    #[graphql(name = "payment_history")]
    pub fn payment_history(&self, select: Option<Select>) -> Vec<CurrentAccountPaymentHistory> {
        CurrentAccountPaymentHistory::new(select, self.account)
    }

    #[graphql(name = "start_date")]
    pub fn start_date(&self) -> Date {
        Date {
            day: self.account.start_date.day as i32,
            month: self.account.start_date.month as i32,
            year: self.account.start_date.year as i32,
        }
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
}
