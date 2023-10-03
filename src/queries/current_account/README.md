# Current Account

```graphql
{
  current_accounts {
    insight {
      count
    }
    current_account {
      company {
        name
      }
      account_number
      payment_history(select: LATEST) {
        age_in_months
        account_balance {
          amount
          currency
        }
        change(since: PREVIOUS) {
          delta
          impact
          polarity
        }
      }
    }
  }
}
```
