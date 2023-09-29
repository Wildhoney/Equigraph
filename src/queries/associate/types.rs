use serde::Deserialize;

use crate::parser::types::{DateField, NameField};

pub struct AssociateObject<'a> {
    pub person: &'a AssociateField,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AssociatesField {
    pub associate: Vec<AssociateField>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AssociateField {
    pub dob: DateField,
    pub name: NameField,
}
