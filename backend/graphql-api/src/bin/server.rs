use csb_db::Context;
use csb_grapql_lib::{mutation::Mutation, query::Query};
use juniper::EmptySubscription;
use rocket::{response::content, State};
use rocket_cors::{AllowedOrigins, CorsOptions, Method};
use std::str::FromStr;
type Schema = csb_grapql_lib::Schema;

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
async fn get_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
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
                Method::from_str("options").unwrap(),
            ]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true);

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let context = Context::new(database_url)
        .await
        .expect("Failed to create context");
    let _ = rocket::build()
        .manage(context)
        .manage(Schema::new(
            Query {},
            Mutation {},
            EmptySubscription::<Context>::new(),
        ))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .attach(cors.to_cors().unwrap())
        .launch()
        .await
        .expect("server to launch");
}
