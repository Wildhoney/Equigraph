use super::fields::AssociateField;
use crate::{
    objects::output::{Date, Name},
    schema::Context,
};
use juniper::FieldResult;

pub struct Associate<'a> {
    pub associate: &'a AssociateField,
}

impl Associate<'_> {
    pub fn new(context: &Context) -> FieldResult<Vec<Associate>> {
        let associates = context.reports.get(0).map(|report| {
            report
                .non_address_specific_data
                .associates
                .associate
                .iter()
                .map(|associate| Associate {
                    associate: &associate,
                })
                .collect::<Vec<_>>()
        });

        match associates {
            Some(associates) => Ok(associates),
            None => Ok(vec![]),
        }
    }
}

#[juniper::graphql_object(context = Context)]
impl Associate<'_> {
    pub fn name(&self) -> Name {
        Name {
            title: &self.associate.name.title,
            forename: &self.associate.name.forename,
            surname: &self.associate.name.surname,
        }
    }

    #[graphql(name = "date_of_birth")]
    pub fn date_of_birth(&self) -> Date {
        Date {
            day: self.associate.dob.day as i32,
            month: self.associate.dob.month as i32,
            year: self.associate.dob.year as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_get_associate_name() {
        let query = r#"
            query Associates {
                associates {
                    associate {
                        name {
                            title
                            forename
                            surname
                        }
                    }
                }
            }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "associates": {"associate": [{"name": {"title": "MRS", "forename": "GIHOY", "surname": "HENYJACI"}}]}
            })
        );
    }

    #[test]
    fn it_can_get_associate_date_of_birth() {
        let query = r#"
            query Associates {
                associates {
                    associate {
                        date_of_birth {
                            day
                            month
                            year
                        }
                    }
                }
            }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "associates": {"associate": [{"date_of_birth": {"day": 7, "month": 2, "year": 1991}}]}
            })
        );
    }
}
