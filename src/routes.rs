use salvo::{
    cors::{self, Cors},
    hyper::Method,
    prelude::*,
};

use crate::blog::auth::{login, register};

pub fn router() -> Router {
    let cors_handler = Cors::new()
        .allow_origin(cors::Any)
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(vec![
            "CONTENT-TYPE",
            "content-type",
            "Access-Control-Request-Method",
            "Access-Control-Allow-Origin",
            "Access-Control-Allow-Headers",
            "Access-Control-Max-Age",
            "authorization",
            "Authorization",
        ])
        .into_handler();

    Router::new()
        .push(
            Router::with_path("/login").post(login)
        ).push(
            Router::with_path("/register").post(register)
        ).push(
        Router::with_path("<**rest>")
            .hoop(cors_handler)
            .options(handler::empty()),
    )
}
