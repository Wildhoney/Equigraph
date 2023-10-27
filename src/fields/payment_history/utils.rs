use super::PaymentHistoryField;
use crate::objects::input::Select;

pub fn select_payment_history(
    select: Select,
    payment_histories: &Vec<PaymentHistoryField>,
) -> &[PaymentHistoryField] {
    match select {
        Select::All => &payment_histories[..],
        Select::Latest => &payment_histories.get(0..1).unwrap_or(&[]),
        Select::Oldest => &payment_histories
            .get(payment_histories.len() - 1..)
            .unwrap_or(&[]),
    }
}
