// src/main.rs

// dependencies
use actix_files as fs;
use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use once_cell::sync::OnceCell;
use tera::{Error, Tera};

// initializer for the compiled template item, uses a static memory location
static COMPILED_TEMPLATE: OnceCell<Tera> = OnceCell::new();

// build the tera template
fn get_index_template() -> Result<&'static Tera, Error> {
    COMPILED_TEMPLATE.get_or_try_init(|| {
        let mut index_template = Tera::new("templates/**/*")?;
        index_template.add_template_file("templates/index.html", None)?;
        Ok(index_template)
    })
}

// function to get the current date
fn get_today() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

// function to get the current time
fn get_time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

// function to set up the tera index template
fn index_template() -> Result<String, Error> {
    let mut context = tera::Context::new();
    context.insert("date", &get_today());
    context.insert("time", &get_time());
    get_index_template()?.render("index.html", &context)
}

// index route endpont handler; serves today's date and time as HTML
async fn index() -> impl Responder {
    match index_template() {
        Ok(template) => HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(template),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::html())
            .body(format!("Error rendering template: {}", e)),
    }
}

// health_check endpoing handler; returns a 200 OK response with an empty body
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// message endpoint handler; returns a simple greeting
async fn message() -> impl Responder {
    let msg = "Brought to you by htmx".to_string();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(msg)
}

// main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health_check", web::get().to(health_check))
            .route("/message", web::get().to(message))
            .service(fs::Files::new("/", "screen.css").use_last_modified(true))
            .service(fs::Files::new("/", "favicon.ico").use_last_modified(true))
            .service(fs::Files::new("/", "htmx.min.js").use_last_modified(true))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
