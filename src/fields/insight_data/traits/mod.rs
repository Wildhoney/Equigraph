use super::{CreditCard, CurrentAccount, InsightField, InsightKind, SecuredLoan, UnsecuredLoan};
use crate::fields::payment_history::utils::PaymentHistory;
use itertools::Itertools;

pub trait Insights {
    fn get_payment_histories(&self) -> Vec<PaymentHistory>;
}

impl Insights for Vec<InsightField<CurrentAccount>> {
    fn get_payment_histories(&self) -> Vec<PaymentHistory> {
        self.iter()
            .map(|current_account| PaymentHistory {
                insight: InsightKind::CurrentAccount(&current_account),
                list: &current_account.payment_history,
            })
            .collect_vec()
    }
}

impl Insights for Vec<InsightField<SecuredLoan>> {
    fn get_payment_histories(&self) -> Vec<PaymentHistory> {
        self.iter()
            .map(|secured_loan| PaymentHistory {
                insight: InsightKind::SecuredLoan(&secured_loan),
                list: &secured_loan.payment_history,
            })
            .collect_vec()
    }
}

impl Insights for Vec<InsightField<UnsecuredLoan>> {
    fn get_payment_histories(&self) -> Vec<PaymentHistory> {
        self.iter()
            .map(|unsecured_loan| PaymentHistory {
                insight: InsightKind::UnsecuredLoan(&unsecured_loan),
                list: &unsecured_loan.payment_history,
            })
            .collect_vec()
    }
}

impl Insights for Vec<InsightField<CreditCard>> {
    fn get_payment_histories(&self) -> Vec<PaymentHistory> {
        self.iter()
            .map(|credit_card| PaymentHistory {
                insight: InsightKind::CreditCard(&credit_card),
                list: &credit_card.payment_history,
            })
            .collect_vec()
    }
}

pub trait Insight {
    fn get_account_number(&self) -> String;
}

impl Insight for InsightField<SecuredLoan> {
    fn get_account_number(&self) -> String {
        self.account_number.to_owned()
    }
}

impl Insight for InsightField<UnsecuredLoan> {
    fn get_account_number(&self) -> String {
        self.account_number.to_owned()
    }
}

impl Insight for InsightField<CurrentAccount> {
    fn get_account_number(&self) -> String {
        self.account_number.to_owned()
    }
}

impl Insight for InsightField<CreditCard> {
    fn get_account_number(&self) -> String {
        self.account_number.to_owned()
    }
}
