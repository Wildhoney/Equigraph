use crate::{
    fields::{NonAddressSpecificDataField, ProviderField, SoleSearchField},
    parser::types::ReportTrait,
    queries::{
        associates::associates::AssociatesField, credit_cards::credit_cards::CreditCards,
        current_accounts::current_accounts::CurrentAccounts, scores::scores::ScoresField,
        secured_loans::secured_loans::SecuredLoans,
        unsecured_loans::unsecured_loans::UnsecuredLoans,
    },
    schema::Context,
    utils::unique_id,
};
use juniper::FieldResult;
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

    pub fn scores(&self) -> FieldResult<&ScoresField> {
        Ok(&self.non_address_specific_data.scores)
    }

    pub fn associates(&self) -> FieldResult<&AssociatesField> {
        Ok(&self.non_address_specific_data.associates)
    }

    #[graphql(name = "current_accounts")]
    pub fn current_accounts(&self) -> FieldResult<CurrentAccounts> {
        Ok(CurrentAccounts {
            report: &self,
            items: self.get_current_accounts(),
        })
    }

    #[graphql(name = "secured_loans")]
    pub fn secured_loans(&self) -> FieldResult<SecuredLoans> {
        Ok(SecuredLoans {
            report: &self,
            items: self.get_secured_loans(),
        })
    }

    #[graphql(name = "unsecured_loans")]
    pub fn unsecured_loans(&self) -> FieldResult<UnsecuredLoans> {
        Ok(UnsecuredLoans {
            report: &self,
            items: self.get_unsecured_loans(),
        })
    }

    #[graphql(name = "credit_cards")]
    pub fn credit_cards(&self) -> FieldResult<CreditCards> {
        Ok(CreditCards {
            report: &self,
            items: self.get_credit_cards(),
        })
    }
}
