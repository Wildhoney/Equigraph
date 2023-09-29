use serde::Deserialize;

use crate::parser::types::{Date, Name};

pub struct AssociateObject<'a> {
    pub person: &'a Associate,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Associates {
    pub associate: Vec<Associate>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Associate {
    pub dob: Date,
    pub name: Name,
}
