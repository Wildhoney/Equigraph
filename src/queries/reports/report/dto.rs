use crate::{
    fields::{NonAddressSpecificDataField, ProviderField, SoleSearchField},
    parser::types::Report,
    utils::unique_id,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Input {
    #[serde(default = "unique_id")]
    pub id: Uuid,
    #[serde(alias = "providerField")]
    pub provider: ProviderField,
    #[serde(alias = "nonAddressSpecificData")]
    pub non_address_specific_data: NonAddressSpecificDataField,
    #[serde(alias = "soleSearch")]
    pub sole_search: SoleSearchField,
}

pub struct Output<'a> {
    pub report: &'a Report,
    pub input: &'a Input,
}
