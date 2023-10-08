use juniper::{FieldResult, GraphQLObject};
use serde::Deserialize;

use crate::{
    fields::matched_address::MatchedAddressField,
    objects::{
        input::Format,
        output::{Balance, Company, CompanyClass, Date},
    },
    parser::fields::{DateField, PaymentFrequencyField, PaymentStatusField},
    schema::Context,
    utils::get_date,
};

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct CurrentAccountField {
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

#[derive(Debug, PartialEq, Deserialize, GraphQLObject, Clone)]
pub struct BalanceField {
    #[serde(alias = "balanceAmount")]
    pub balance_amount: AmountField,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLObject, Clone)]
pub struct CreditLimitField {
    pub limit: AmountField,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLObject, Clone)]
pub struct AmountField {
    pub amount: i32,
    pub currency: String,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct PaymentHistoryField {
    #[serde(alias = "accountBalance")]
    pub account_balance: BalanceField,
    #[serde(alias = "ageInMonths")]
    pub age_in_months: i32,
    #[serde(alias = "paymentStatus")]
    pub payment_status: PaymentStatusField,
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
        Balance {
            amount: self.current_balance.balance_amount.amount,
            currency: &self.current_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "default_balance")]
    pub fn default_balance(&self) -> Balance {
        Balance {
            amount: self.default_balance.balance_amount.amount,
            currency: &self.default_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "start_balance")]
    pub fn start_balance(&self) -> Balance {
        Balance {
            amount: self.start_balance.balance_amount.amount,
            currency: &self.start_balance.balance_amount.currency,
        }
    }

    #[graphql(name = "credit_limit")]
    pub fn credit_limit(&self) -> Option<Balance> {
        match self.credit_limit {
            Some(ref credit_limit) => Some(Balance {
                amount: credit_limit.limit.amount,
                currency: &credit_limit.limit.currency,
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

    // #[graphql(name = "payment_history")]
    // pub fn payment_history(&self, select: Option<Select>) -> Vec<CurrentAccountPaymentHistory> {
    //     CurrentAccountPaymentHistory::new(select, self.current_account)
    // }

    // pub fn address(&self) -> FieldResult<&MatchedAddressField> {
    //     self.address
    // }
}
