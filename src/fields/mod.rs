pub mod associate;
pub mod current_account;
pub mod score;

use juniper::GraphQLEnum;
use serde::Deserialize;

use self::{associate::Associates, current_account::CurrentAccount, score::Scores};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Report {
    #[serde(alias = "nonAddressSpecificData")]
    pub non_address_specific_data: NonAddressSpecificData,
    #[serde(alias = "soleSearch")]
    pub sole_search: SoleSearch,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct NonAddressSpecificData {
    pub associates: Associates,
    pub scores: Scores,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SoleSearch {
    pub primary: Primary,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Primary {
    #[serde(alias = "suppliedAddressData")]
    pub supplied_address_data: Vec<SuppliedAddressData>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SuppliedAddressData {
    #[serde(alias = "matchedAddress")]
    pub matched_address: MatchedAddress,
    #[serde(alias = "addressSpecificData")]
    pub address_specific_data: AddressSpecificData,
    #[serde(alias = "noticeOfCorrectionOrDisputePresent")]
    pub notice_of_correction_or_dispute_present: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct MatchedAddress {
    address: Address,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Address {
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
pub struct AddressSpecificData {
    #[serde(alias = "insightData")]
    pub insight_data: InsightData,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct InsightData {
    #[serde(alias = "currentAccount")]
    pub current_account: Vec<CurrentAccount>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Name {
    pub title: String,
    pub forename: String,
    pub surname: String,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum)]
pub enum PaymentFrequency {
    #[serde(alias = "MONTHLY")]
    Monthly,
    #[serde(alias = "PERIODICALLY")]
    Periodically,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum)]
pub enum PaymentStatus {
    #[serde(alias = "ZERO")]
    Zero,
    S,
    U,
}
