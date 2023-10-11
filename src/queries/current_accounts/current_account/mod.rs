use crate::{
    fields::{
        matched_address::MatchedAddressField, payment_history::PaymentHistoryField, BalanceField,
        CreditLimitField, DateField, PaymentFrequencyField,
    },
    objects::{
        input::{Format, Select},
        output::{Balance, Company, CompanyClass, Date},
    },
    schema::Context,
    utils::{get_date, get_formatted_currency, unique_id},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct CurrentAccountField {
    #[serde(default = "unique_id")]
    pub id: Uuid,
    #[serde(alias = "accountNumber")]
    pub account_number: String,
    #[serde(alias = "currentBalance")]
    pub current_balance: BalanceField,
    #[serde(alias = "creditLimit")]
    pub credit_limit: Option<CreditLimitField>,
    #[serde(alias = "defaultBalance")]
    pub default_balance: BalanceField,
    #[serde(alias = "startBalance")]
    pub start_balance: BalanceField,
    pub overdraft: bool,
    #[serde(alias = "paymentFrequency")]
    pub payment_frequency: PaymentFrequencyField,
    #[serde(alias = "companyName")]
    pub company_name: String,
    #[serde(alias = "companyClass")]
    pub company_class: CompanyClass,
    #[serde(alias = "paymentHistory")]
    pub payment_history: Vec<PaymentHistoryField>,
    #[serde(alias = "startDate")]
    pub start_date: DateField,
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountField {
    #[graphql(name = "account_number")]
    pub fn account_number(&self) -> &String {
        &self.account_number
    }

    pub fn company(&self) -> Company {
        Company {
            kind: &self.company_class,
            name: &self.company_name,
        }
    }

    #[graphql(name = "has_overdraft")]
    pub fn has_overdraft(&self) -> &bool {
        &self.overdraft
    }

    #[graphql(name = "current_balance")]
    pub fn current_balance(&self) -> Balance {
        let amount = self.current_balance.balance_amount.amount;
        let currency = &self.current_balance.balance_amount.currency;

        Balance {
            amount,
            currency,
            value: get_formatted_currency(amount, currency),
        }
    }

    #[graphql(name = "default_balance")]
    pub fn default_balance(&self) -> Balance {
        let amount = self.current_balance.balance_amount.amount;
        let currency = &self.default_balance.balance_amount.currency;

        Balance {
            amount,
            currency,
            value: get_formatted_currency(amount, currency),
        }
    }

    #[graphql(name = "start_balance")]
    pub fn start_balance(&self) -> Balance {
        let amount = self.current_balance.balance_amount.amount;
        let currency = &self.start_balance.balance_amount.currency;

        Balance {
            amount,
            currency,
            value: get_formatted_currency(amount, currency),
        }
    }

    #[graphql(name = "credit_limit")]
    pub fn credit_limit(&self) -> Option<Balance> {
        match &self.credit_limit {
            Some(credit_limit) => Some(Balance {
                amount: credit_limit.limit.amount,
                currency: &credit_limit.limit.currency,
                value: get_formatted_currency(
                    credit_limit.limit.amount,
                    &credit_limit.limit.currency,
                ),
            }),
            None => None,
        }
    }

    #[graphql(name = "payment_frequency")]
    pub fn payment_frequency(&self) -> &PaymentFrequencyField {
        &self.payment_frequency
    }

    #[graphql(name = "start_date")]
    pub fn start_date(&self, format: Format) -> Option<Date> {
        get_date(
            self.start_date.year,
            self.start_date.month,
            self.start_date.day,
            format,
        )
    }

    #[graphql(name = "payment_history")]
    pub fn payment_history(&self, select: Select) -> &[PaymentHistoryField] {
        match select {
            Select::All => &self.payment_history[..],
            Select::Latest => &self.payment_history.get(0..1).unwrap_or(&[]),
            Select::Oldest => &self
                .payment_history
                .get(self.payment_history.len() - 1..)
                .unwrap_or(&[]),
        }
    }

    pub fn address(&self, context: &Context) -> Option<&MatchedAddressField> {
        let address = context.reports.iter().find_map(|report| {
            report
                .sole_search
                .primary
                .supplied_address_data
                .iter()
                .find(|supplied_address_data| {
                    supplied_address_data
                        .address_specific_data
                        .insight_data
                        .current_account
                        .iter()
                        .any(|current_account| current_account.id == self.id)
                })
        })?;

        Some(&address.matched_address)
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_get_current_account_details() {
        let query = r#"
            query CurrentAccount {
                current_accounts {
                    current_account {
                      account_number
                      company {
                        kind
                        name
                      }
                      current_balance {
                        amount
                        currency
                      }
                      default_balance {
                        amount
                        currency
                      }
                      start_balance {
                        amount
                        currency
                      }
                      credit_limit {
                        amount
                        currency
                      }
                      payment_frequency
                      start_date(format: "%d/%m/%Y")
                      address {
                        number
                        street
                        town
                        county
                      }
                    }
                }
            }
        "#;

        let expected = graphql_value!({
            "current_accounts": {
                "current_account": [
                    {
                    "account_number": "zGML/Ld93it5j86rAFo2wxM8oGHNdoWJj4WTwoRmkcc=",
                    "company": {
                        "kind": "BANK",
                        "name": "HSBC PLC (I)"
                    },
                    "current_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "default_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "start_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "credit_limit": {juniper::Value::Null},
                    "payment_frequency": "MONTHLY",
                    "start_date": "10/11/2004",
                    "address": {
                        "number": "25447",
                        "street": "LZOQYQFI GYYW",
                        "town": "HORSHAM",
                        "county": "PIQW GYHZIF"
                    }
                    },
                    {
                    "account_number": "3oEmu6B1FCnWguuTc93gXTPtT3NMcaxCKSm2MLOFvMw=",
                    "company": {
                        "kind": "BANK",
                        "name": "LLOYDS BANK (WAS LLOYDS TSB) (I)"
                    },
                    "current_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "default_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "start_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "credit_limit": {
                        "amount": 1000,
                        "currency": "GBP"
                    },
                    "payment_frequency": "MONTHLY",
                    "start_date": "28/06/2013",
                    "address": {
                        "number": "25447",
                        "street": "LZOQYQFI GYYW",
                        "town": "HORSHAM",
                        "county": "PIQW GYHZIF"
                    }
                    },
                    {
                    "account_number": "5YduNv4WxF4SOS0GqS8uh/yOA/TFTgsQT1uH5kAB8RQ=",
                    "company": {
                        "kind": "BANK",
                        "name": "LLOYDS BANK (WAS LLOYDS TSB) (I)"
                    },
                    "current_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "default_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "start_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "credit_limit": {juniper::Value::Null},
                    "payment_frequency": "MONTHLY",
                    "start_date": "06/06/2016",
                    "address": {
                        "number": "25447",
                        "street": "LZOQYQFI GYYW",
                        "town": "HORSHAM",
                        "county": "PIQW GYHZIF"
                    }
                    },
                    {
                    "account_number": "iMt7bI9kNQtpjsWYsMr69lgUsgyg5XQVMF4dhBknm3E=",
                    "company": {
                        "kind": "BANK",
                        "name": "MONZO BANK LIMITED (I)"
                    },
                    "current_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "default_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "start_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "credit_limit": {juniper::Value::Null},
                    "payment_frequency": "PERIODICALLY",
                    "start_date": "29/09/2019",
                    "address": {
                        "number": "25447",
                        "street": "LZOQYQFI GYYW",
                        "town": "HORSHAM",
                        "county": "PIQW GYHZIF"
                    }
                    },
                    {
                    "account_number": "LFTlUsWtDduQAo2L7zJOtNKDI86DztlNPL6Fg7iz4+M=",
                    "company": {
                        "kind": "BANK",
                        "name": "MONZO BANK LIMITED (I)"
                    },
                    "current_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "default_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "start_balance": {
                        "amount": 0,
                        "currency": "GBP"
                    },
                    "credit_limit": {
                        "amount": 1000,
                        "currency": "GBP"
                    },
                    "payment_frequency": "PERIODICALLY",
                    "start_date": "17/07/2019",
                    "address": {
                        "number": "25447",
                        "street": "LZOQYQFI GYYW",
                        "town": "HORSHAM",
                        "county": "PIQW GYHZIF"
                    }
                    }
                ]
            }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}
