use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<String> {
    Html(dioxus_ssr::render_lazy(rsx! {
        div {
            h1 { "Hello World!" }
            p { "greetings from dioxus"}
            (0..10).map(|f| rsx!{
                p { "abc: {f}" }
            })
        }
    }))
}
