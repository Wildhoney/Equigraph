use super::{associate::AssociateField, AssociatesRoot};
use crate::schema::Context;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct AssociatesField {
    pub associate: Vec<AssociateField>,
}

impl AssociatesField {
    pub fn new(context: &Context) -> AssociatesRoot {
        AssociatesRoot { context }
    }
}
