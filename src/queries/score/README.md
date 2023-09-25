# Score

```graphql
{
  old_score: score(kind: RNOLF04) {
    current
    maximum
    change(since: PREVIOUS) {
      score(kind: RNOLF04) {
        current
        maximum
      }
    }
  }
  new_score: score(kind: PSOLF01) {
    current
    maximum
    change(since: PREVIOUS) {
      delta
      polarity
      impact
    }
  }
}
```