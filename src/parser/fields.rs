use crate::{
    fields::matched_address::MatchedAddressField,
    queries::{
        associates::associates::AssociatesField,
        current_accounts::current_accounts::CurrentAccountField, scores::scores::ScoresField,
    },
    schema::Context,
};
use itertools::Itertools;
use juniper::GraphQLEnum;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ReportField {
    #[serde(alias = "nonAddressSpecificData")]
    pub non_address_specific_data: NonAddressSpecificDataField,
    #[serde(alias = "soleSearch")]
    pub sole_search: SoleSearchField,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct NonAddressSpecificDataField {
    pub associates: AssociatesField,
    pub scores: ScoresField,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SoleSearchField {
    pub primary: PrimaryField,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct PrimaryField {
    #[serde(alias = "suppliedAddressData")]
    pub supplied_address_data: Vec<SuppliedAddressDataField>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SuppliedAddressDataField {
    #[serde(alias = "matchedAddress")]
    pub matched_address: MatchedAddressField,
    #[serde(alias = "addressSpecificData")]
    pub address_specific_data: AddressSpecificDataField,
    #[serde(alias = "noticeOfCorrectionOrDisputePresent")]
    pub notice_of_correction_or_dispute_present: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AddressSpecificDataField {
    #[serde(alias = "insightData")]
    pub insight_data: InsightDataField,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct InsightDataField {
    #[serde(alias = "currentAccount")]
    pub current_account: Vec<CurrentAccountField>,
}

pub enum InsightDataFieldKind {
    CurrentAccount,
}

impl InsightDataField {
    pub fn new(context: &Context, _kind: InsightDataFieldKind) -> Self {
        Self {
            current_account: context
                .reports
                .iter()
                .flat_map(|report| {
                    report
                        .sole_search
                        .primary
                        .supplied_address_data
                        .iter()
                        .flat_map(|supplied_address_data| {
                            supplied_address_data
                                .address_specific_data
                                .insight_data
                                .current_account
                                .to_owned()
                        })
                        .collect::<Vec<_>>()
                })
                .unique_by(|current_account| current_account.account_number.to_owned())
                .collect::<Vec<_>>(),
        }
    }
}

#[juniper::graphql_object(context = Context)]
impl InsightDataField {
    #[graphql(name = "current_account")]
    pub fn current_account(&self) -> Vec<&CurrentAccountField> {
        self.current_account.iter().collect::<Vec<_>>()
    }
}

#[derive(Debug, PartialEq, Deserialize, Eq, Hash, Clone)]
pub struct DateField {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[derive(Debug, PartialEq, Deserialize, Eq, Hash, Clone)]
pub struct NameField {
    pub title: String,
    pub forename: String,
    pub surname: String,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum, Clone)]
pub enum PaymentFrequencyField {
    #[serde(alias = "MONTHLY")]
    Monthly,
    #[serde(alias = "PERIODICALLY")]
    Periodically,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum, Clone, Copy)]
pub enum PaymentStatusField {
    #[serde(alias = "ZERO")]
    Zero,
    S,
    U,
}
