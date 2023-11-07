pub mod dto;

use self::dto::Output;
use crate::{
    fields::{ProviderField, SuppliedAddressDataField},
    queries::{associates::associates::AssociatesField, scores::scores::ScoresField},
    schema::Context,
};

#[juniper::graphql_object(context = Context)]
impl Output<'_> {
    pub fn provider(&self) -> &ProviderField {
        &self.input.provider
    }

    pub fn scores(&self) -> &ScoresField {
        &self.input.non_address_specific_data.scores
    }

    pub fn associates(&self) -> &AssociatesField {
        &self.input.non_address_specific_data.associates
    }

    pub fn addresses(&self) -> &Vec<SuppliedAddressDataField> {
        &self.input.sole_search.primary.supplied_address_data
    }
}
