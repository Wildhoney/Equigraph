use super::{CreditCard, CurrentAccount, InsightField, SecuredLoan, UnsecuredLoan};

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
