pub mod mutation;
pub mod query;
pub mod subscription;

pub type Schema =
    juniper::RootNode<'static, query::Query, mutation::Mutation, subscription::Subscription>;
