# Current Account

```graphql
{
  current_accounts {
    insight {
      count
    }
    list {
      company_name
      account_number
      has_overdraft
      current_balance {
        amount
        currency
      }
      default_balance {
        amount
        currency
      }
      payment_frequency
    }
  }
}
```
