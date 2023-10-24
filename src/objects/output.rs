use juniper::{GraphQLEnum, GraphQLObject};
use serde::Deserialize;

use crate::schema::Context;

pub type Date = String;

#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct Name<'a> {
    pub title: &'a str,
    pub forename: &'a str,
    pub surname: &'a str,
}

#[derive(Debug, PartialEq, GraphQLEnum)]
pub enum Sentiment {
    High,
    Medium,
    Low,
}

#[derive(Debug, PartialEq, GraphQLEnum)]
pub enum Polarity {
    Unchanged,
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, GraphQLEnum)]
pub enum Impact {
    None,
    High,
    Low,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum, Clone)]
pub enum CompanyClass {
    #[serde(alias = "AC")]
    Accountancy,
    #[serde(alias = "BF")]
    BankNonCLSBAndFinanceDivisions,
    #[serde(alias = "BK")]
    Bank,
    #[serde(alias = "BS")]
    BuildingSociety,
    #[serde(alias = "CB")]
    CreditBroker,
    #[serde(alias = "CC")]
    CreditCard,
    #[serde(alias = "CD")]
    ConsumerDirect,
    #[serde(alias = "CE")]
    CustomsAndExcise,
    #[serde(alias = "CG")]
    ChequeGuarantor,
    #[serde(alias = "CH")]
    ChargeCard,
    #[serde(alias = "CI")]
    CreditInsurer,
    #[serde(alias = "CL")]
    ConsumerLetter,
    #[serde(alias = "CR")]
    CommercialReporting,
    #[serde(alias = "CS")]
    CommunicationSupplier,
    #[serde(alias = "CV")]
    ConsumerVerification,
    #[serde(alias = "DC")]
    DebtCollector,
    #[serde(alias = "DI")]
    DistributionAndWholesalers,
    #[serde(alias = "EA")]
    EmploymentAgency,
    #[serde(alias = "EN")]
    EnquiryAgent,
    #[serde(alias = "ES")]
    EnergySupplier,
    #[serde(alias = "FN")]
    FinanceHouse,
    #[serde(alias = "FS")]
    FinancialServices,
    #[serde(alias = "GI")]
    GeneralInsurance,
    #[serde(alias = "GV")]
    Government,
    #[serde(alias = "HC")]
    HireCarRental,
    #[serde(alias = "HF")]
    HomeFurnisher,
    #[serde(alias = "HI")]
    HomeImprovement,
    #[serde(alias = "HO")]
    HouseBuilder,
    #[serde(alias = "HS")]
    HealthServices,
    #[serde(alias = "HT")]
    HotelAndTravel,
    #[serde(alias = "IB")]
    InsuranceBroker,
    #[serde(alias = "IN")]
    Insurance,
    #[serde(alias = "IR")]
    InlandRevenue,
    #[serde(alias = "LA")]
    LossAdjuster,
    #[serde(alias = "LG")]
    Leasing,
    #[serde(alias = "MD")]
    MotorDealer,
    #[serde(alias = "ME")]
    Media,
    #[serde(alias = "MF")]
    ManufacturingIndustrial,
    #[serde(alias = "MK")]
    Marketing,
    #[serde(alias = "MN")]
    #[serde(alias = "CZ")]
    Miscellaneous,
    #[serde(alias = "MO")]
    MailOrder,
    #[serde(alias = "MS")]
    MortgageSupplier,
    #[serde(alias = "OS")]
    Overseas,
    #[serde(alias = "PM")]
    PropertyManagement,
    #[serde(alias = "PO")]
    Police,
    #[serde(alias = "PU")]
    PublicUtility,
    #[serde(alias = "RN")]
    Rental,
    #[serde(alias = "RT")]
    Retailer,
    #[serde(alias = "SB")]
    StockBroker,
    #[serde(alias = "SO")]
    Solicitor,
    #[serde(alias = "SR")]
    SlotRental,
    #[serde(alias = "SS")]
    SecurityServices,
    #[serde(alias = "TC")]
    TVProgrammeSupplier,
    #[serde(alias = "TI")]
    TravelInsurer,
    #[serde(alias = "TP")]
    Thirdparty,
    #[serde(alias = "TV")]
    TVRental,
    #[serde(alias = "XX")]
    Training,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct Company<'a> {
    pub kind: &'a CompanyClass,
    pub name: &'a String,
}

#[derive(Debug)]
pub struct PaymentAnalysis {
    pub active: bool,
    pub total: i32,
    pub payments: i32,
}

#[juniper::graphql_object(context = Context)]
impl PaymentAnalysis {
    pub fn made(&self) -> i32 {
        self.payments
    }

    pub fn remaining(&self) -> Option<i32> {
        if self.active == false {
            return None;
        }

        Some((self.total - self.payments as i32) as i32)
    }

    pub fn total(&self) -> Option<i32> {
        if self.active == false {
            return None;
        }

        Some(self.total)
    }
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum, Clone)]
pub enum CreditLimitChange {
    #[serde(alias = "DECREASE")]
    Decrease,
    #[serde(alias = "INCREASE")]
    Increase,
    #[serde(alias = "NO_CHANGE")]
    NoChange,
    #[serde(alias = "UNKNOWN")]
    Unknown,
}
