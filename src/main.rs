use axum::{
    routing::{get, post},
    Router,
};
use std::env;
use std::net::SocketAddr;

mod routes;
mod template;

#[derive(Clone)]
pub struct AppState {
    public_url: String,
}

#[tokio::main]
async fn main() {
    let port = match env::var("PORT") {
        Ok(val) => val.parse::<u16>().unwrap(),
        Err(_e) => 3000,
    };

    let public_url = match env::var("PUBLIC_URL") {
        Ok(val) => val.trim_end_matches('/').to_string(),
        Err(_e) => "http://0.0.0.0:3000".to_string(),
    };

    let state = AppState { public_url };

    // build our application with a route
    let app = Router::new()
        .route("/", get(routes::index))
        .route("/hello", post(routes::hello))
        .route("/headers", get(routes::get_headers))
        .with_state(state);

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
