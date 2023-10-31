use super::traits::Insight;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct Changes<T> {
    pub added: Vec<T>,
    pub removed: Vec<T>,
}

impl<T> Changes<T>
where
    T: Insight + Clone,
{
    pub fn new<'a>(insights: Vec<T>, compare_with_insights: Vec<T>) -> Changes<T> {
        let insight_ids = insights
            .iter()
            .map(|item| item.get_account_number())
            .collect_vec();

        let compare_with_insight_ids = compare_with_insights
            .iter()
            .map(|item| item.get_account_number())
            .collect_vec();

        let added = insights
            .clone()
            .into_iter()
            .filter(|insight| !compare_with_insight_ids.contains(&&insight.get_account_number()))
            .collect_vec();

        let removed = insights
            .clone()
            .into_iter()
            .filter(|insight| !insight_ids.contains(&&insight.get_account_number()))
            .collect_vec();

        Changes { added, removed }
    }
}
