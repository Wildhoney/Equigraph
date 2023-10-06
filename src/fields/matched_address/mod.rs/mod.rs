pub mod fields;

use self::fields::MatchedAddressField;
use crate::schema::Context;
use juniper::FieldResult;

#[derive(Debug, PartialEq)]
pub struct Address<'a> {
    pub address: &'a MatchedAddressField,
}

impl Address<'_> {
    pub fn new<'a>(address: &'a MatchedAddressField) -> FieldResult<Address<'a>> {
        Ok(Address { address })
    }
}

#[juniper::graphql_object(context = Context)]
impl Address<'_> {
    pub fn number(&self) -> &String {
        &self.address.address.number
    }

    pub fn street(&self) -> &String {
        &self.address.address.street1
    }

    pub fn town(&self) -> &String {
        &self.address.address.post_town
    }

    pub fn postcode(&self) -> &String {
        &self.address.address.postcode
    }

    pub fn county(&self) -> &String {
        &self.address.address.county
    }
}
