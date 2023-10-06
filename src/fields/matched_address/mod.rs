use crate::schema::Context;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct MatchedAddressField {
    pub address: AddressField,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AddressField {
    #[serde(alias = "addressID")]
    pub address_id: String,
    pub county: String,
    pub number: String,
    #[serde(alias = "postTown")]
    pub post_town: String,
    pub postcode: String,
    pub street1: String,
}

#[juniper::graphql_object(context = Context)]
impl MatchedAddressField {
    pub fn number(&self) -> &String {
        &self.address.number
    }

    pub fn street(&self) -> &String {
        &self.address.street1
    }

    pub fn town(&self) -> &String {
        &self.address.post_town
    }

    pub fn postcode(&self) -> &String {
        &self.address.postcode
    }

    pub fn county(&self) -> &String {
        &self.address.county
    }
}
