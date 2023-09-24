use serde::Deserialize;

use crate::queries::{associates::types::Associates, score::types::Scores};

pub type Reports = Vec<Option<Report>>;

#[derive(Debug, Deserialize)]
pub struct Report {
    #[serde(alias = "nonAddressSpecificData")]
    non_address_specific_data: NonAddressSpecificData,
}

#[derive(Debug, Deserialize)]
struct NonAddressSpecificData {
    associates: Option<Associates>,
    scores: Option<Scores>,
}

#[derive(Debug, Deserialize)]
pub struct Date {
    day: u8,
    month: u8,
    year: u16,
}

#[derive(Debug, Deserialize)]
pub struct Name {
    title: String,
    forename: String,
    surname: String,
}
