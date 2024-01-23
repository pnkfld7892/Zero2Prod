use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};


async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

// #[tokio::main]
pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
