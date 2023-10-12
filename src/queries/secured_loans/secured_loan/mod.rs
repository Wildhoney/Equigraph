use crate::{
    fields::{
        payment_history::PaymentHistoryField, AmountField, BalanceField, DateField, LoanTypeField,
        PaymentFrequencyField,
    },
    objects::{
        input::Select,
        output::{Company, CompanyClass},
    },
    schema::Context,
    utils::{partition_payment_history, unique_id},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct SecuredLoanField {
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
    #[serde(alias = "loanType")]
    pub loan_type: LoanTypeField,
    #[serde(alias = "startDate")]
    pub start_date: DateField,
    #[serde(alias = "paymentFrequency")]
    pub payment_frequency: PaymentFrequencyField,
    #[serde(alias = "companyName")]
    pub company_name: String,
    #[serde(alias = "companyClass")]
    pub company_class: CompanyClass,
    pub flexible: bool,
}

#[juniper::graphql_object(context = Context)]
impl SecuredLoanField {
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

    #[graphql(name = "payment_history")]
    pub fn payment_history(&self, select: Select) -> &[PaymentHistoryField] {
        partition_payment_history(select, &self.payment_history)
    }

    #[graphql(name = "loan_type")]
    pub fn loan_type(&self) -> &LoanTypeField {
        &self.loan_type
    }

    pub fn flexible(&self) -> bool {
        self.flexible
    }

    #[graphql(name = "start_date")]
    pub fn start_date(&self) -> &DateField {
        &self.start_date
    }
}
