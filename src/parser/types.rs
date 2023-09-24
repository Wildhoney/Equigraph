use serde::Deserialize;

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
struct Scores {
    score: Vec<Score>,
}

#[derive(Debug, Deserialize)]
struct Score {
    positive: bool,
    #[serde(alias = "scoreLabel")]
    score_label: String,
    #[serde(alias = "sourcedFrom")]
    sourced_from: String,
    value: u8,
}

#[derive(Debug, Deserialize)]
struct Associates {
    associate: Vec<Associate>,
}

#[derive(Debug, Deserialize)]
struct Associate {
    dob: Date,
    name: Name,
}

#[derive(Debug, Deserialize)]
struct Date {
    day: u8,
    month: u8,
    year: u16,
}

#[derive(Debug, Deserialize)]
struct Name {
    title: String,
    forename: String,
    surname: String,
}
