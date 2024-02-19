use actix_web::{
    web::{self, Form},
    HttpResponse,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use tracing;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: Form<FormData>, _pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    tracing::info!("request_id {} - Add '{}' '{}' as a new subscrimbo.",request_id,form.email,form.name);
    match  sqlx::query!(
        r#"
        INSERT INTO subscriptions(id,email,name,subscribed_at)
        VALUES ($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(_pool.get_ref())
    .await{
        Ok(_) =>{
            tracing::info!("request_id {} - New subscriber saved successfully!",request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query: {:?}",request_id,e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
