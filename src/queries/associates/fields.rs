use serde::Deserialize;

use crate::parser::fields;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Associates {
    pub associate: Vec<Associate>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Associate {
    pub dob: fields::Date,
    pub name: fields::Name,
}
