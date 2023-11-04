use crate::{
    fields::{
        NonAddressSpecificDataField, ProviderField, SoleSearchField, SuppliedAddressDataField,
    },
    queries::{associates::associates::AssociatesField, scores::scores::ScoresField},
    schema::Context,
    utils::unique_id,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ReportField {
    #[serde(default = "unique_id")]
    pub id: Uuid,
    #[serde(alias = "providerField")]
    pub provider: ProviderField,
    #[serde(alias = "nonAddressSpecificData")]
    pub non_address_specific_data: NonAddressSpecificDataField,
    #[serde(alias = "soleSearch")]
    pub sole_search: SoleSearchField,
}

#[juniper::graphql_object(context = Context)]
impl ReportField {
    pub fn provider(&self) -> &ProviderField {
        &self.provider
    }

    pub fn scores(&self) -> &ScoresField {
        &self.non_address_specific_data.scores
    }

    pub fn associates(&self) -> &AssociatesField {
        &self.non_address_specific_data.associates
    }

    pub fn addresses(&self) -> &Vec<SuppliedAddressDataField> {
        &self.sole_search.primary.supplied_address_data
    }
}
