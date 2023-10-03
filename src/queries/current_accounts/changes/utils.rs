use crate::{
    fields,
    objects::{self},
};

pub fn get_delta<'a>(
    since: &'a objects::input::Since,
    account: &'a fields::current_account::CurrentAccount,
    payment_history: &'a fields::current_account::PaymentHistory,
) -> Option<i32> {
    let current_index = account
        .payment_history
        .iter()
        .position(|x| x == payment_history)?;

    let current_amount = payment_history.account_balance.balance_amount.amount;

    match since {
        objects::input::Since::First => match account.payment_history.last() {
            Some(last) => Some(current_amount - last.account_balance.balance_amount.amount),
            _ => None,
        },
        objects::input::Since::Previous => match account
            .payment_history
            .get(current_index as usize + 1)
        {
            Some(previous) => Some(current_amount - previous.account_balance.balance_amount.amount),
            _ => None,
        },
        objects::input::Since::Next => {
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
    since: &'a objects::input::Since,
    account: &'a fields::current_account::CurrentAccount,
    payment_history: &'a fields::current_account::PaymentHistory,
) -> Option<objects::output::Polarity> {
    let delta = get_delta(&since, &account, &payment_history);

    match delta {
        Some(delta) => match delta {
            delta if delta == 0 => Some(objects::output::Polarity::Unchanged),
            delta if delta > 0 => Some(objects::output::Polarity::Negative),
            delta if delta < 500 => Some(objects::output::Polarity::Positive),
            _ => Some(objects::output::Polarity::Unchanged),
        },
        _ => None,
    }
}

pub fn get_impact<'a>(
    since: &'a objects::input::Since,
    account: &'a fields::current_account::CurrentAccount,
    payment_history: &'a fields::current_account::PaymentHistory,
) -> Option<objects::output::Impact> {
    let delta = get_delta(&since, &account, &payment_history);

    match delta {
        Some(delta) => match delta {
            delta if delta == 0 => Some(objects::output::Impact::None),
            delta if delta > 1_000 || 1_000 < delta => Some(objects::output::Impact::High),
            _ => Some(objects::output::Impact::Low),
        },
        _ => None,
    }
}
