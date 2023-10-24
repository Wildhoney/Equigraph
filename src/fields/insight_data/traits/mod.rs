use super::InsightKind;
use crate::{
    fields::payment_history::utils::PaymentHistory,
    queries::{
        current_accounts::current_account::CurrentAccountField,
        secured_loans::secured_loan::SecuredLoanField,
        unsecured_loans::unsecured_loan::UnsecuredLoanField,
    },
};
use itertools::Itertools;

pub trait Insights {
    fn get_payment_histories(&self) -> Vec<PaymentHistory>;
}

impl Insights for Vec<CurrentAccountField> {
    fn get_payment_histories(&self) -> Vec<PaymentHistory> {
        self.iter()
            .map(|current_account| PaymentHistory {
                insight: InsightKind::CurrentAccount(&current_account),
                list: &current_account.payment_history,
            })
            .collect_vec()
    }
}

impl Insights for Vec<SecuredLoanField> {
    fn get_payment_histories(&self) -> Vec<PaymentHistory> {
        self.iter()
            .map(|secured_loan| PaymentHistory {
                insight: InsightKind::SecuredLoan(&secured_loan),
                list: &secured_loan.payment_history,
            })
            .collect_vec()
    }
}

impl Insights for Vec<UnsecuredLoanField> {
    fn get_payment_histories(&self) -> Vec<PaymentHistory> {
        self.iter()
            .map(|unsecured_loan| PaymentHistory {
                insight: InsightKind::UnsecuredLoan(&unsecured_loan),
                list: &unsecured_loan.payment_history,
            })
            .collect_vec()
    }
}

pub trait Insight {
    fn get_account_number(&self) -> String;
}

impl Insight for SecuredLoanField {
    fn get_account_number(&self) -> String {
        self.account_number.to_owned()
    }
}

impl Insight for UnsecuredLoanField {
    fn get_account_number(&self) -> String {
        self.account_number.to_owned()
    }
}

impl Insight for CurrentAccountField {
    fn get_account_number(&self) -> String {
        self.account_number.to_owned()
    }
}
