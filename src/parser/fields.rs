use juniper::GraphQLEnum;
use serde::Deserialize;

use crate::queries::{
    associates::fields::AssociatesField, current_accounts::fields::CurrentAccountField,
    score::fields::ScoresField,
};

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
pub struct MatchedAddressField {
    address: AddressField,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AddressField {
    #[serde(alias = "addressID")]
    address_id: String,
    county: String,
    number: String,
    #[serde(alias = "postTown")]
    post_town: String,
    postcode: String,
    street1: String,
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

#[derive(Debug, PartialEq, Deserialize)]
pub struct DateField {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct NameField {
    pub title: String,
    pub forename: String,
    pub surname: String,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum)]
pub enum PaymentFrequencyField {
    #[serde(alias = "MONTHLY")]
    Monthly,
    #[serde(alias = "PERIODICALLY")]
    Periodically,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum)]
pub enum PaymentStatusField {
    #[serde(alias = "ZERO")]
    Zero,
    S,
    U,
}
