pub mod insight_data;
pub mod matched_address;
pub mod payment_history;

use self::insight_data::InsightDataField;
use crate::{
    fields::matched_address::MatchedAddressField,
    objects::{
        input::{EndingZeroes, Like},
        output::Date,
    },
    queries::{associates::associates::AssociatesField, scores::scores::ScoresField},
    schema::Context,
    utils::{get_amount, get_date},
};
use juniper::GraphQLEnum;
use serde::Deserialize;

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

#[derive(Debug, PartialEq, Deserialize, Eq, Hash, Clone)]
pub struct DateField {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[juniper::graphql_object(context = Context)]
impl DateField {
    pub fn day(&self) -> i32 {
        self.day as i32
    }

    pub fn month(&self) -> i32 {
        self.month as i32
    }

    pub fn year(&self) -> i32 {
        self.year as i32
    }

    pub fn formatted(&self, like: Like) -> Option<Date> {
        get_date(self.year, self.month, self.day, like)
    }
}

#[derive(Debug, PartialEq, Deserialize, Eq, Hash, Clone)]
pub struct NameField {
    pub title: String,
    pub forename: String,
    pub surname: String,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum, Clone)]
pub enum PaymentFrequencyField {
    #[serde(alias = "WEEKLY")]
    Weekly,
    #[serde(alias = "FORTNIGHTLY")]
    Fortnightly,
    #[serde(alias = "MONTHLY")]
    Monthly,
    #[serde(alias = "QUARTERLY")]
    Quarterly,
    #[serde(alias = "BI_ANNUAL")]
    BiAnnually,
    #[serde(alias = "ANNUAL")]
    Annually,
    #[serde(alias = "PERIODICALLY")]
    Periodically,
    #[serde(alias = "OTHER")]
    Other,
    #[serde(alias = "UNKNOWN")]
    Unknown,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum, Clone)]
pub enum LoanTypeField {
    #[serde(alias = "MORTGAGE")]
    Mortgage,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum, Clone, Copy)]
pub enum PaymentStatusField {
    #[serde(alias = "Q")]
    AccountInQuery,
    #[serde(alias = "R")]
    Repossession,
    #[serde(alias = "V")]
    VoluntaryRepossession,
    #[serde(alias = "N")]
    Inactive,
    #[serde(alias = "D")]
    Default,
    #[serde(alias = "Z")]
    NotTakenUp,
    #[serde(alias = "S")]
    Settled,
    #[serde(alias = "I")]
    AgreedPayments,
    #[serde(alias = "X")]
    Terminated,
    #[serde(alias = "U")]
    Unknown,
    #[serde(alias = "W")]
    WrittenOff,
    #[serde(alias = "G")]
    GoneAway,
    #[serde(alias = "DOT")]
    NoUpdateReceived,
    #[serde(alias = "B")]
    BadArrears,
    #[serde(alias = "A")]
    ModerateArrears,
    #[serde(alias = "ZERO")]
    UpToDate,
    #[serde(alias = "ONE")]
    OneMonthArrears,
    #[serde(alias = "TWO")]
    TwoMonthsArrears,
    #[serde(alias = "THREE")]
    ThreeMonthsArrears,
    #[serde(alias = "FOUR")]
    FourMonthsArrears,
    #[serde(alias = "FIVE")]
    FiveMonthsArrears,
    #[serde(alias = "SIX")]
    SixMonthsArrears,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct BalanceField {
    #[serde(alias = "balanceAmount")]
    pub balance_amount: AmountField,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct CreditLimitField {
    pub limit: AmountField,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct AmountField {
    pub amount: i32,
    pub currency: String,
}

#[juniper::graphql_object(context = Context)]
impl AmountField {
    pub fn amount(&self) -> i32 {
        self.amount
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn formatted(&self, zeroes: Option<EndingZeroes>) -> String {
        let amount = self.amount;
        let currency = &self.currency;
        get_amount(amount, currency, zeroes)
    }
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct FixedPaymentTermsField {
    #[serde(alias = "numberOfPayments")]
    pub number_of_payments: i32,
    #[serde(alias = "paymentAmount")]
    pub payment_amount: AmountField,
}

#[juniper::graphql_object(context = Context)]
impl FixedPaymentTermsField {
    #[graphql(name = "number_of_payments")]
    pub fn number_of_payments(&self) -> i32 {
        self.number_of_payments
    }

    #[graphql(name = "payment_amount")]
    pub fn payment_amount(&self) -> &AmountField {
        &self.payment_amount
    }
}
