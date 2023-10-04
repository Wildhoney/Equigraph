use super::{fields::ScoreLabelField, utils::get_sentiment};
use crate::{objects::output::Sentiment, parser::types::Report, schema::Context};

#[derive(Debug, PartialEq)]
pub struct ScoreInsight<'a> {
    pub kind: &'a ScoreLabelField,
    pub report: Option<&'a Report>,
}

#[juniper::graphql_object(context = Context)]
impl ScoreInsight<'_> {
    pub fn sentiment(&self) -> Option<Sentiment> {
        get_sentiment(&self.kind, &self.report)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mocks::get_parsed_reports,
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
}
