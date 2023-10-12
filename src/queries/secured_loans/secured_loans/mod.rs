use super::secured_loan::SecuredLoanField;
use crate::schema::Context;

pub struct SecuredLoans {
    pub items: Vec<SecuredLoanField>,
}

#[juniper::graphql_object(context = Context)]
impl SecuredLoans {
    #[graphql(name = "secured_loan")]
    pub fn current_account() -> &Vec<SecuredLoanField> {
        &self.items
    }
}
