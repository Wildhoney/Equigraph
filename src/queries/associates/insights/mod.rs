use crate::{parser::types::Report, schema::Context};
use juniper::FieldResult;

pub struct Insights<'a> {
    pub report: &'a Option<&'a Report>,
}

impl Insights<'_> {
    pub fn new<'a>(report: &'a Option<&Report>) -> FieldResult<Insights<'a>> {
        Ok(Insights { report })
    }
}

#[juniper::graphql_object(context = Context)]
impl Insights<'_> {
    #[graphql(name = "count")]
    pub fn count(&self) -> Option<i32> {
        match self.report {
            Some(report) => {
                Some(report.non_address_specific_data.associates.associate.len() as i32)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_display_insights() {
        let query = r#"
            query Associates {
                associates {
                    insights {
                        count
                    }
                }
            }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "associates": {"insights": { "count": 1 }}
            })
        );
    }
}
