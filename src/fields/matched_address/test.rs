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
