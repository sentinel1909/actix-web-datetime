// src/main.rs

// dependencies
use actix_files as fs;
use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use std::sync::OnceLock;
use tera::Tera;

static COMPILED_TEMPLATE: OnceLock<Tera> = OnceLock::new();

fn get_index_template() -> &'static Tera {
    COMPILED_TEMPLATE.get_or_init(|| {
        let mut index_template = Tera::new("templates/**/*").unwrap();
        index_template
            .add_template_file("templates/index.html", None)
            .unwrap();
        index_template
    })
}

// function to get the current date and time
fn get_today() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
   
}

// function to set up the tera index template
fn index_template() -> String {
    let mut context = tera::Context::new();
    context.insert("date", &get_today());
    get_index_template().render("index.html", &context).unwrap()
}

// index route endpont handler; serves today's date and time as HTML
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(index_template())
}

// health_check endpoing handler; returns a 200 OK response with an empty body
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(fs::Files::new("/", "screen.css").use_last_modified(true))
            .service(fs::Files::new("/", "favicon.ico").use_last_modified(true))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
