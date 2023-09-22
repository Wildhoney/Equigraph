use juniper::{GraphQLObject,EmptySubscription, FieldResult, RootNode, EmptyMutation, GraphQLInputObject, GraphQLEnum};

#[derive(GraphQLObject)]
#[graphql(description = "")]
struct Report {
    current: i32,
    maximum: i32,
}

pub struct QueryRoot;

#[derive(GraphQLInputObject)]
#[graphql(description = "")]
struct Score {
    kind: String,
}

#[derive(GraphQLEnum)]
pub enum ScoreKind {
    RNOLF04,
    PSOLF01
}

#[juniper::graphql_object]
impl QueryRoot {
    fn score(kind: ScoreKind) -> FieldResult<Report> {
        Ok(Report {
            current: 968,
            maximum: match kind {
                ScoreKind::RNOLF04 => 700,
                ScoreKind::PSOLF01 => 1000,
            }
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}