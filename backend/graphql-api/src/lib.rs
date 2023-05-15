use csb_db::Context;

pub mod mutation;
pub mod query;

pub type Schema = juniper::RootNode<
    'static,
    query::Query,
    mutation::Mutation,
    juniper::EmptySubscription<Context>,
>;
