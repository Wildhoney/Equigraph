# Associates

```graphql
{
  associates {
    insights {
      count
    }
    associate(unique: true) {
      name{
        title
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
