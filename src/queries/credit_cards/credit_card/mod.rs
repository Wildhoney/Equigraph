use crate::{
    fields::{
        insight_data::{CreditCard, InsightField},
        matched_address::MatchedAddressField,
        payment_history::{PartitionPaymentHistory, PaymentHistoryField},
        AmountField, BalanceField, DateField, PaymentFrequencyField,
    },
    objects::{
        input::Select,
        output::{Company, CompanyClass},
    },
    schema::Context,
    utils::{find_address_by_id, unique_id},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct CreditCardField {
    #[serde(default = "unique_id")]
    pub id: Uuid,
    #[serde(alias = "accountNumber")]
    pub account_number: String,
    #[serde(alias = "currentBalance")]
    pub current_balance: BalanceField,
    #[serde(alias = "defaultBalance")]
    pub default_balance: BalanceField,
    #[serde(alias = "startBalance")]
    pub start_balance: BalanceField,
    #[serde(alias = "paymentHistory")]
    pub payment_history: Vec<PaymentHistoryField>,
    #[serde(alias = "startDate")]
    pub start_date: DateField,
    #[serde(alias = "lastUpdateDate")]
    pub last_update_date: DateField,
    #[serde(alias = "endDate")]
    pub end_date: Option<DateField>,
    #[serde(alias = "paymentFrequency")]
    pub payment_frequency: PaymentFrequencyField,
    #[serde(alias = "companyName")]
    pub company_name: String,
    #[serde(alias = "companyClass")]
    pub company_class: CompanyClass,
}

#[juniper::graphql_object(name = "CreditCard", context = Context)]
impl InsightField<CreditCard> {
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

    #[graphql(name = "current_balance")]
    pub fn current_balance(&self) -> &AmountField {
        &self.current_balance.balance_amount
    }

    #[graphql(name = "default_balance")]
    pub fn default_balance(&self) -> &AmountField {
        &self.default_balance.balance_amount
    }

    #[graphql(name = "payment_frequency")]
    pub fn payment_frequency(&self) -> &PaymentFrequencyField {
        &self.payment_frequency
    }

    #[graphql(name = "start_balance")]
    pub fn start_balance(&self) -> &AmountField {
        &self.start_balance.balance_amount
    }

    #[graphql(name = "start_date")]
    pub fn start_date(&self) -> &DateField {
        &self.start_date
    }

    #[graphql(name = "last_update_date")]
    pub fn update_date(&self) -> &DateField {
        &self.last_update_date
    }

    #[graphql(name = "end_date")]
    pub fn end_date(&self) -> &Option<DateField> {
        &self.end_date
    }

    pub fn address(&self, context: &Context) -> Option<&MatchedAddressField> {
        let address = find_address_by_id(self.id, &context.reports)?;
        Some(&address.matched_address)
    }

    #[graphql(name = "payment_history")]
    pub fn payment_history(&self, select: Select) -> &[PaymentHistoryField] {
        self.payment_history.partition(select)
    }
}
