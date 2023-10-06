# Associates

```graphql
{
  associates {
    insights {
      associates_count
    }
    associate {
      name {
        title
        forename
        surname
      }
      date_of_birth(format: "%Y-%m-%d")
    }
  }
}

# insight {
#   age
#   known_since
# }
# electoral_roll {
# ...
# }
```
