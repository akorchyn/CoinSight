use csb_db::Context;
use juniper::graphql_object;

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    fn api_version() -> &'static str {
        "1.0"
    }
}
