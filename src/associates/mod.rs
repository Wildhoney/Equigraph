use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "")]
pub struct NameObject {
    pub title: String,
    pub forename: String,
    pub surname: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "")]
pub struct DateOfBirthObject {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

#[derive(GraphQLObject)]
#[graphql(description = "")]
pub struct AssociateObject {
    pub name: NameObject,

    #[graphql(name = "date_of_birth")]
    pub date_of_birth: DateOfBirthObject,
}
