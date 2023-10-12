use crate::{schema::Context, utils::unique_id};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct SecuredLoanField {
    #[serde(default = "unique_id")]
    pub id: Uuid,
    #[serde(alias = "accountNumber")]
    pub account_number: String,
}

#[juniper::graphql_object(context = Context)]
impl SecuredLoanField {
    #[graphql(name = "account_number")]
    pub fn account_number(&self) -> &String {
        &self.account_number
    }
}
