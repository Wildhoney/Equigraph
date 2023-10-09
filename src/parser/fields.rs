use crate::{
    fields::matched_address::MatchedAddressField,
    objects::output::Balance,
    queries::{
        associates::associates::AssociatesField,
        current_accounts::{
            current_account::CurrentAccountField, current_accounts::CurrentAccounts,
        },
        scores::scores::ScoresField,
    },
    schema::Context,
};
use itertools::Itertools;
use juniper::{GraphQLEnum, GraphQLObject};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ReportField {
    #[serde(alias = "nonAddressSpecificData")]
    pub non_address_specific_data: NonAddressSpecificDataField,
    #[serde(alias = "soleSearch")]
    pub sole_search: SoleSearchField,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct NonAddressSpecificDataField {
    pub associates: AssociatesField,
    pub scores: ScoresField,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SoleSearchField {
    pub primary: PrimaryField,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct PrimaryField {
    #[serde(alias = "suppliedAddressData")]
    pub supplied_address_data: Vec<SuppliedAddressDataField>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SuppliedAddressDataField {
    #[serde(alias = "matchedAddress")]
    pub matched_address: MatchedAddressField,
    #[serde(alias = "addressSpecificData")]
    pub address_specific_data: AddressSpecificDataField,
    #[serde(alias = "noticeOfCorrectionOrDisputePresent")]
    pub notice_of_correction_or_dispute_present: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AddressSpecificDataField {
    #[serde(alias = "insightData")]
    pub insight_data: InsightDataField,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct InsightDataField {
    #[serde(alias = "currentAccount")]
    pub current_account: Vec<CurrentAccountField>,
}

pub enum InsightDataFieldKind {
    CurrentAccount,
}

impl InsightDataField {
    pub fn new(context: &Context, _kind: InsightDataFieldKind) -> CurrentAccounts {
        CurrentAccounts {
            items: context
                .reports
                .iter()
                .flat_map(|report| {
                    report
                        .sole_search
                        .primary
                        .supplied_address_data
                        .iter()
                        .flat_map(|supplied_address_data| {
                            supplied_address_data
                                .address_specific_data
                                .insight_data
                                .current_account
                                .to_owned()
                        })
                        .collect::<Vec<_>>()
                })
                .unique_by(|current_account| current_account.account_number.to_owned())
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Eq, Hash, Clone)]
pub struct DateField {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[derive(Debug, PartialEq, Deserialize, Eq, Hash, Clone)]
pub struct NameField {
    pub title: String,
    pub forename: String,
    pub surname: String,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum, Clone)]
pub enum PaymentFrequencyField {
    #[serde(alias = "MONTHLY")]
    Monthly,
    #[serde(alias = "PERIODICALLY")]
    Periodically,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum, Clone, Copy)]
pub enum PaymentStatusField {
    #[serde(alias = "ZERO")]
    Zero,
    S,
    U,
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
impl PaymentHistoryField {
    #[graphql(name = "age_in_months")]
    pub fn age_in_months(&self) -> i32 {
        self.age_in_months
    }

    #[graphql(name = "payment_status")]
    pub fn payment_status(&self) -> &PaymentStatusField {
        &self.payment_status
    }

    #[graphql(name = "account_balance")]
    pub fn account_balance(&self) -> Balance {
        Balance {
            amount: self.account_balance.balance_amount.amount,
            currency: &self.account_balance.balance_amount.currency,
        }
    }

    // pub fn changes(&self, since: Since) -> CurrentAccountChanges {
    //     CurrentAccountChanges {
    //         since: since.to_owned(),
    //         account: &self.account,
    //         payment_history: &self.payment_history,
    //     }
    // }
}
