use crate::states::FormData;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgConnection;
use std::sync::Mutex;
use uuid::Uuid;

pub async fn subrcribe(
    form: web::Form<FormData>,
    connection: web::Data<Mutex<PgConnection>>,
) -> impl Responder {
    let mut conn = connection.lock();
    let _ = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(conn.as_deref_mut().unwrap())
    .await;
    HttpResponse::Ok()
}
