use serde::Deserialize;

use crate::queries::{associates::types::Associates, score::types::Scores};

pub type Reports = Vec<Report>;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Report {
    #[serde(alias = "nonAddressSpecificData")]
    pub non_address_specific_data: NonAddressSpecificData,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct NonAddressSpecificData {
    pub associates: Associates,
    pub scores: Scores,
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
