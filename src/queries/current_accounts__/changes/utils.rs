use crate::{
    objects::{
        input::Since,
        output::{Impact, Polarity},
    },
    queries::current_accounts::fields::{CurrentAccountField, PaymentHistoryField},
};

pub fn get_delta<'a>(
    since: &'a Since,
    account: &'a CurrentAccountField,
    payment_history: &'a PaymentHistoryField,
) -> Option<i32> {
    let current_index = account
        .payment_history
        .iter()
        .position(|x| x == payment_history)?;

    let current_amount = payment_history.account_balance.balance_amount.amount;

    match since {
        Since::First => match account.payment_history.last() {
            Some(last) => Some(current_amount - last.account_balance.balance_amount.amount),
            _ => None,
        },
        Since::Previous => match account.payment_history.get(current_index as usize + 1) {
            Some(previous) => Some(current_amount - previous.account_balance.balance_amount.amount),
            _ => None,
        },
        Since::Next => {
            if current_index == 0 {
                return None;
            }

            match account.payment_history.get(current_index as usize - 1) {
                Some(next) => Some(current_amount - next.account_balance.balance_amount.amount),
                _ => None,
            }
        }
    }
}

pub fn get_polarity<'a>(
    since: &'a Since,
    account: &'a CurrentAccountField,
    payment_history: &'a PaymentHistoryField,
) -> Option<Polarity> {
    let delta = get_delta(&since, &account, &payment_history);

    match delta {
        Some(delta) => match delta {
            delta if delta == 0 => Some(Polarity::Unchanged),
            delta if delta > 0 => Some(Polarity::Negative),
            delta if delta < 500 => Some(Polarity::Positive),
            _ => Some(Polarity::Unchanged),
        },
        _ => None,
    }
}

pub fn get_impact<'a>(
    since: &'a Since,
    account: &'a CurrentAccountField,
    payment_history: &'a PaymentHistoryField,
) -> Option<Impact> {
    let delta = get_delta(&since, &account, &payment_history);

    match delta {
        Some(delta) => match delta {
            delta if delta == 0 => Some(Impact::None),
            delta if delta > 1_000 || 1_000 < delta => Some(Impact::High),
            _ => Some(Impact::Low),
        },
        _ => None,
    }
}
