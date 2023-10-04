use serde::Deserialize;

use crate::parser::fields::{DateField, NameField};

#[derive(Debug, PartialEq, Deserialize)]
pub struct AssociatesField {
    pub associate: Vec<AssociateField>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AssociateField {
    pub dob: DateField,
    pub name: NameField,
}
