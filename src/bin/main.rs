// src/main.rs

// dependencies
use actix_files as fs;
use actix_web::{
    http::header::ContentType, web, App, HttpResponse, HttpServer, Responder, ResponseError,
};
use actix_web_datetime_lib::routes::health_check;
use chrono::prelude::*;
use once_cell::sync::OnceCell;
use tera::Tera;

// strut to wrap a tera::Error type
struct TemplateRenderError(tera::Error);

// implement the ResponseError trait for our TemplateRenderError type
impl ResponseError for TemplateRenderError {}

// implement the Rust standard error trait for TemplateRenderError
impl std::error::Error for TemplateRenderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

// implement the Debug trait for TemplateRenderError
impl std::fmt::Debug for TemplateRenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nCaused by:\n\t{}", self, self.0)
    }
}

// implement the Display trait for our TemplateRenderError type
impl std::fmt::Display for TemplateRenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A template rendering error was encountered while trying to build the Tera template."
        )
    }
}

// implement the From trait to convert a tera::Error into a TemplateRenderError
impl From<tera::Error> for TemplateRenderError {
    fn from(err: tera::Error) -> Self {
        TemplateRenderError(err)
    }
}

// initializer for the compiled template item, uses a static memory location
static COMPILED_TEMPLATE: OnceCell<Tera> = OnceCell::new();

// build the tera template
fn get_index_template() -> Result<&'static Tera, TemplateRenderError> {
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
fn index_template() -> Result<String, TemplateRenderError> {
    let mut context = tera::Context::new();
    context.insert("date", &get_today());
    context.insert("time", &get_time());
    Ok(get_index_template()?.render("index.html", &context)?)
}

// index route endpont handler; serves today's date and time as HTML
async fn index() -> impl Responder {
    match index_template() {
        Ok(template) => HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(template),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::html())
            .body(format!("Error: {}", e)),
    }
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
            .route("/message", web::get().to(message))
            .service(health_check)
            .service(fs::Files::new("/", "screen.css").use_last_modified(true))
            .service(fs::Files::new("/", "favicon.ico").use_last_modified(true))
            .service(fs::Files::new("/", "htmx.min.js").use_last_modified(true))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
