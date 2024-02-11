use actix_web::{HttpRequest,HttpResponse};

pub async fn health_check(_req: HttpRequest) -> HttpResponse{
    dbg!(&"healthcheck!");
    HttpResponse::Ok().finish()
}
