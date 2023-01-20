use axum::{body::Body, extract, extract::State, http::Request, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::template::*;
use crate::AppState;

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let template = IndexTemplate {
        public_url: state.public_url,
    };
    HtmlTemplate(template)
}

#[derive(Debug, Serialize)]
pub struct HelloResponse {
    message: String,
}

#[derive(Debug, Deserialize)]
pub struct HelloRequest {
    name: String,
    greeting: Option<String>,
}

pub async fn hello(extract::Json(payload): extract::Json<HelloRequest>) -> Json<HelloResponse> {
    let mut greet = "Hello".into();
    if let Some(g) = payload.greeting {
        if !g.is_empty() {
            greet = g;
        }
    };

    let msg = HelloResponse {
        message: format!("{} {}!", greet, payload.name),
    };

    Json(msg)
}

pub async fn get_headers(req: Request<Body>) -> Json<HashMap<String, String>> {
    let mut hres: HashMap<String, String> = HashMap::new();

    for (name, value) in req.headers() {
        hres.insert(name.to_string(), value.to_str().unwrap().to_string());
    }
    Json(hres)
}
