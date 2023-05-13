use juniper::graphql_object;

use crate::Context;

pub struct Subscription;

#[graphql_object(context = Context)]
impl Subscription {
    fn api_version() -> &'static str {
        "1.0"
    }

    // fn price_updated(context: &Context, symbol: String) -> FieldResult<Price> {
    //     PriceUpdated::new(context)
    // }
}
