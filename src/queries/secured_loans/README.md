# Secured Loans

```graphql
query SecuredLoans {
  secured_loans {
    secured_loan {
      account_number
      payment_frequency
      flexible
      start_balance {
        amount
      }
      fixed_payment_terms {
        number_of_payments
        payment_amount {
          formatted(zeroes: STRIP)
        }
      }
      start_date {
        year
      }
      end_date {
        year
      }
      insights {
        active
        current_end_date {
          formatted(like: "%d/%m/%Y")
        }
        payment_analysis {
          made
          total
          remaining
        }
      }
    }
  }
}
```
