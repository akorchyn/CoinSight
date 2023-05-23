use csb_grapql_lib::{mutation::Mutation, query::Query, Context};
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

    let database_crypto =
        std::env::var("DATABASE_CRYPTO_URL").expect("DATABASE_CRYPTO_URL must be set");
    let database_users =
        std::env::var("DATABASE_USERS_URL").expect("DATABASE_USERS_URL must be set");
    let user_service = std::env::var("USER_SERVICE_URL").expect("USER_SERVICE_URL must be set");

    let database_crypto = csb_db_crypto::Db::new(database_crypto)
        .await
        .expect("Failed to create crypto context");
    let database_users = csb_db_user::Db::new(database_users)
        .await
        .expect("Failed to create users context");
    let user_service = csb_grapql_lib::UserService::new(user_service);

    let context = Context {
        crypto_db: database_crypto,
        user_db: database_users,
        user_service,
    };

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
