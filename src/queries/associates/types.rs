use serde::Deserialize;

use crate::parser::types::{Date, Name};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Associates {
    associate: Vec<Associate>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Associate {
    dob: Date,
    name: Name,
}
