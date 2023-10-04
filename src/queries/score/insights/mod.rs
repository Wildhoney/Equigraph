use juniper::FieldResult;

use super::{fields::ScoreLabelField, utils::get_sentiment};
use crate::{objects::output::Sentiment, parser::types::Report, schema::Context};

#[derive(Debug, PartialEq)]
pub struct ScoreInsight<'a> {
    pub kind: &'a ScoreLabelField,
    pub report: Option<&'a Report>,
}

impl ScoreInsight<'_> {
    pub fn new<'a>(
        kind: &'a ScoreLabelField,
        report: Option<&'a Report>,
    ) -> FieldResult<ScoreInsight<'a>> {
        Ok(ScoreInsight { kind, report })
    }
}

#[juniper::graphql_object(context = Context)]
impl ScoreInsight<'_> {
    pub fn sentiment(&self) -> Option<Sentiment> {
        get_sentiment(&self.kind, &self.report)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use juniper::graphql_value;

    use crate::{
        mocks::{get_parsed_reports, run_graphql_query},
        objects::output::Sentiment,
        queries::score::{fields::ScoreLabelField, utils::get_sentiment},
    };

    #[test]
    fn it_can_compute_score_sentiment() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_sentiment(&ScoreLabelField::PSOLF01, &reports.get(0)),
            Some(Sentiment::High)
        );
    }

    #[test]
    fn it_can_display_insights() {
        let query = r#"
            query Score {
                score(kind: PSOLF01) {
                    insights {
                        sentiment
                    }
                }
            }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "score": {"insights": { "sentiment": "HIGH" }}
            })
        );
    }
}
