# Current Accounts

```graphql
{
  current_accounts {
    insights {
      count
    }
    current_account {
      company {
        name
      }
      account_number
      start_date(format: "%Y-%m-%d")
      payment_history(select: LATEST) {
        age_in_months
        account_balance {
          amount
          currency
        }
        changes(since: PREVIOUS) {
          delta
          impact
          polarity
        }
      }
    }
  }
}
```
