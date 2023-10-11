pub mod insight_data;
pub mod matched_address;
pub mod payment_history;

use self::insight_data::InsightDataField;
use crate::{
    fields::matched_address::MatchedAddressField,
    queries::{associates::associates::AssociatesField, scores::scores::ScoresField},
};
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
