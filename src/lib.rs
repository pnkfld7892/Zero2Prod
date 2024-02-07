use std::net::TcpListener;

use actix_web::web::Form;
use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};
use actix_web::dev::Server;

#[derive(serde::Deserialize)]
struct FormData{
    email: String,
    name: String
}


async fn health_check(_req: HttpRequest) -> HttpResponse{
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: Form<FormData>) -> HttpResponse{
    HttpResponse::Ok().finish()
}

// #[tokio::main]
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    // .bind(address)?
    .listen(listener)?
    .run();
    Ok(server)
}
