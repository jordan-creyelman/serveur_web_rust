use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::middleware::Logger;
use actix_web::error::ResponseError;
use std::fmt;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum FooError {
    Bar,
}

impl fmt::Display for FooError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "display message")
    }
}

impl ResponseError for FooError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().finish()
    }
}

async fn foo_route() -> Result<HttpResponse, FooError> {
    Err(FooError::Bar)
}

async fn index_route() -> impl Responder {
    let mut file = File::open("index.html").expect("Cannot open index.html");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Cannot read index.html");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(contents)
}

async fn about_route() -> impl Responder {
    let mut file = File::open("about.html").expect("Cannot open about.html");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Cannot read about.html");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(contents)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/error", web::get().to(foo_route))
            .route("/", web::get().to(index_route))
            .route("/about", web::get().to(about_route))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
