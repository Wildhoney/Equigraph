use crate::fields;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Associates {
    pub associate: Vec<Associate>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Associate {
    pub dob: fields::Date,
    pub name: fields::Name,
}
