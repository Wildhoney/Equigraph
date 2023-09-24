use serde::Deserialize;

use crate::parser::types::{Date, Name};

#[derive(Debug, Deserialize)]
pub struct Associates {
    associate: Vec<Associate>,
}

#[derive(Debug, Deserialize)]
pub struct Associate {
    dob: Date,
    name: Name,
}
