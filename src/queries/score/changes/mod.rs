mod utils;

use self::utils::{get_delta, get_impact, get_polarity};
use super::{fields::ScoreLabelField, Score};
use crate::{
    objects::{
        input::Since,
        output::{Impact, Polarity},
    },
    parser::{
        types::Report,
        utils::{backward_by, forward_by},
    },
    schema::Context,
};
use juniper::FieldResult;

pub fn fetch<'a>(
    kind: &'a ScoreLabelField,
    report: Option<&'a Report>,
    context: &'a Context,
    since: Since,
) -> FieldResult<ScoreChange<'a>> {
    Ok(ScoreChange {
        kind,
        report: match since {
            Since::First => context.reports.last(),
            Since::Previous => forward_by(1, report, &context.reports),
            Since::Next => backward_by(1, report, &context.reports),
        },
        parent_report: report,
    })
}

pub struct ScoreChange<'a> {
    pub kind: &'a ScoreLabelField,
    pub report: Option<&'a Report>,
    pub parent_report: Option<&'a Report>,
}

#[juniper::graphql_object(context = Context)]
impl ScoreChange<'_> {
    pub fn delta(&self) -> Option<i32> {
        get_delta(&self.kind, &self.report, &self.parent_report)
    }

    pub fn impact(&self) -> Option<Impact> {
        get_impact(&self.kind, &self.report, &self.parent_report)
    }

    pub fn polarity(&self) -> Option<Polarity> {
        get_polarity(&self.kind, &self.report, &self.parent_report)
    }

    pub fn score(&self, kind: ScoreLabelField) -> FieldResult<Score> {
        Ok(Score {
            kind,
            report: self.report,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mocks::get_parsed_reports,
        objects::output::{Impact, Polarity},
        queries::score::{
            changes::utils::{get_delta, get_impact, get_polarity},
            fields::ScoreLabelField,
        },
    };

    #[test]
    fn it_can_compute_score_delta() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_delta(&ScoreLabelField::PSOLF01, &reports.get(1), &reports.get(0)),
            Some(20)
        );
    }

    #[test]
    fn it_can_compute_score_polarity() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_polarity(&ScoreLabelField::PSOLF01, &reports.get(1), &reports.get(0)),
            Some(Polarity::Positive)
        );
    }

    #[test]
    fn it_can_compute_score_impact() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_impact(&ScoreLabelField::PSOLF01, &reports.get(1), &reports.get(0)),
            Some(Impact::Low)
        );
    }
}
