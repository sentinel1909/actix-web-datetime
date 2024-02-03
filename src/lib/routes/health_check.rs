// src/lib/routes/health_check.rs

// dependencies
use actix_web::{get, HttpResponse, Responder};

// health_check endpoing handler; returns a 200 OK response with an empty body
#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
