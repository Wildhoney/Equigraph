use super::fields::{self};
use super::{changes::CurrentAccountChanges, fields::CurrentAccountField};
use crate::objects::input::{Select, Since};
use crate::objects::output::Balance;
use crate::{parser::fields::PaymentStatusField, schema::Context};

#[derive(Debug, PartialEq)]
pub struct CurrentAccountPaymentHistory<'a> {
    pub select: Option<Select>,
    pub account: &'a CurrentAccountField,
    pub payment_history: &'a fields::PaymentHistoryField,
}

impl CurrentAccountPaymentHistory<'_> {
    pub fn new<'a>(
        select: Option<Select>,
        account: &'a CurrentAccountField,
    ) -> Vec<CurrentAccountPaymentHistory> {
        let mut payment_histories = account.payment_history.iter();

        let payment_history = match select {
            Some(Select::Latest) => match payment_histories.nth(0) {
                Some(payment_history) => vec![payment_history],
                _ => vec![],
            },
            Some(Select::Oldest) => match payment_histories.last() {
                Some(payment_history) => vec![payment_history],
                _ => vec![],
            },
            Some(Select::Polar) => match [payment_histories.nth(0), payment_histories.last()] {
                [Some(first), Some(last)] => vec![first, last],
                _ => vec![],
            },
            _ => payment_histories.collect::<Vec<_>>(),
        };

        payment_history
            .iter()
            .map(|payment_history| CurrentAccountPaymentHistory {
                select: select.to_owned(),
                account,
                payment_history: payment_history,
            })
            .collect::<Vec<_>>()
    }
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountPaymentHistory<'_> {
    #[graphql(name = "age_in_months")]
    pub fn age_in_months(&self) -> i32 {
        self.payment_history.age_in_months
    }

    #[graphql(name = "payment_status")]
    pub fn payment_status(&self) -> &PaymentStatusField {
        &self.payment_history.payment_status
    }

    #[graphql(name = "account_balance")]
    pub fn account_balance(&self) -> Balance {
        Balance {
            amount: self.payment_history.account_balance.balance_amount.amount,
            currency: &self.payment_history.account_balance.balance_amount.currency,
        }
    }

    pub fn changes(&self, since: Since) -> CurrentAccountChanges {
        CurrentAccountChanges {
            since: since.to_owned(),
            account: &self.account,
            payment_history: &self.payment_history,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_get_payment_history_using_latest_selector() {
        let query = r#"
            query PaymentHistory {
                current_accounts {
                    current_account {
                        payment_history(select: LATEST) {
                            age_in_months
                        }
                    }
                }
            }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "current_accounts": {
                    "current_account": [
                        { "payment_history": [{ "age_in_months": 0 }] },
                        { "payment_history": [{ "age_in_months": 0 }] },
                        { "payment_history": [{ "age_in_months": 0 }] },
                        { "payment_history": [{ "age_in_months": 0 }] },
                        { "payment_history": [{ "age_in_months": 0 }] },
                    ]
                },
            })
        );
    }

    #[test]
    fn it_can_get_payment_history_using_polar_selector() {
        let query = r#"
            query PaymentHistory {
                current_accounts {
                    current_account {
                        payment_history(select: POLAR) {
                            age_in_months
                        }
                    }
                }
            }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "current_accounts": {
                    "current_account": [
                        { "payment_history": [{ "age_in_months": 0 }, { "age_in_months": 47 }] },
                        { "payment_history": [{ "age_in_months": 0 }, { "age_in_months": 47 }] },
                        { "payment_history": [{ "age_in_months": 0 }, { "age_in_months": 47 }] },
                        { "payment_history": [{ "age_in_months": 0 }, { "age_in_months": 27 }] },
                        { "payment_history": [{ "age_in_months": 0 }, { "age_in_months": 27 }] },
                    ]
                },
            })
        );
    }

    #[test]
    fn it_can_get_payment_history_using_oldest_selector() {
        let query = r#"
            query PaymentHistory {
                current_accounts {
                    current_account {
                        payment_history(select: OLDEST) {
                            age_in_months
                        }
                    }
                }
            }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "current_accounts": {
                    "current_account": [
                        { "payment_history": [{ "age_in_months": 47 }] },
                        { "payment_history": [{ "age_in_months": 47 }] },
                        { "payment_history": [{ "age_in_months": 47 }] },
                        { "payment_history": [{ "age_in_months": 27 }] },
                        { "payment_history": [{ "age_in_months": 27 }] },
                    ]
                },
            })
        );
    }
}