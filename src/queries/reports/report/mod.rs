use crate::{
    fields::{
        insight_data::utils::get_insights_from_report, NonAddressSpecificDataField, ProviderField,
        SoleSearchField,
    },
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
            items: get_insights_from_report(self, &|insight_data| &insight_data.current_account),
        })
    }

    #[graphql(name = "secured_loans")]
    pub fn secured_loans(&self) -> FieldResult<SecuredLoans> {
        Ok(SecuredLoans {
            report: &self,
            items: get_insights_from_report(self, &|insight_data| &insight_data.secured_loan),
        })
    }

    #[graphql(name = "unsecured_loans")]
    pub fn unsecured_loans(&self) -> FieldResult<UnsecuredLoans> {
        Ok(UnsecuredLoans {
            report: &self,
            items: get_insights_from_report(self, &|insight_data| &insight_data.unsecured_loan),
        })
    }

    #[graphql(name = "credit_cards")]
    pub fn credit_cards(&self) -> FieldResult<CreditCards> {
        Ok(CreditCards {
            report: &self,
            items: get_insights_from_report(self, &|insight_data| &insight_data.credit_card),
        })
    }
}
