use csb_grapql_lib::{mutation::Mutation, query::Query, subscription::Subscription, Context};
use rocket::{response::content, State};
use rocket_cors::{AllowedOrigins, CorsOptions, Method};
use std::str::FromStr;
type Schema = csb_grapql_lib::Schema;

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(schema, context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(schema, context)
}

#[rocket::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![
                Method::from_str("get").unwrap(),
                Method::from_str("post").unwrap(),
                Method::from_str("patch").unwrap(),
            ]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true);

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let context = Context::new(database_url).expect("Failed to create context");
    let _ = rocket::build()
        .manage(context)
        .manage(Schema::new(Query {}, Mutation {}, Subscription {}))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .attach(cors.to_cors().unwrap())
        .launch()
        .await
        .expect("server to launch");
}
