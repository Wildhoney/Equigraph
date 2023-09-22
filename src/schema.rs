use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

use crate::{
    associates::{AssociateObject, DateOfBirthObject, NameObject},
    score::{ScoreKind, ScoreObject},
};

pub struct QueryRoot;

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}

#[juniper::graphql_object]
impl QueryRoot {
    fn score(kind: ScoreKind) -> FieldResult<ScoreObject> {
        Ok(ScoreObject { kind })
    }
    fn associates() -> FieldResult<Vec<AssociateObject>> {
        Ok(vec![AssociateObject {
            name: NameObject {
                title: "Mr".to_string(),
                forename: "Adam".to_string(),
                surname: "Timberlake".to_string(),
            },
            date_of_birth: DateOfBirthObject {
                day: 10,
                month: 10,
                year: 1985,
            },
        }])
    }
}
