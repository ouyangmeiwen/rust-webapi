use actix_web::web;
use actix_web::Responder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}
//http://127.0.0.1:8000/health
pub async fn health() -> impl Responder {
    web::Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}
