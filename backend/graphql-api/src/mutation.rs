use juniper::graphql_object;

use crate::Context;

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    fn api_version() -> &'static str {
        "1.0"
    }
}
