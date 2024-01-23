use std::net::TcpListener;

use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;


async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

// #[tokio::main]
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    // .bind(address)?
    .listen(listener)?
    .run();
    Ok(server)
}
