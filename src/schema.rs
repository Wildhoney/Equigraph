use juniper::{EmptySubscription, RootNode, EmptyMutation};


pub struct QueryRoot;


pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}