use crate::states::FormData;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

pub async fn subrcribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    let request_id = Uuid::new_v4();
    let req_span = tracing::info_span!("Adding a new subscriber",%request_id, subscriber_email = %form.email,subrcribe_name = %form.name);
    let _req_span_guard = req_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            // This error log falls outside of query_span, we'll rectify it later.
            tracing::error!("request id {} - Failed to execute query: {e:?}", request_id);
            HttpResponse::InternalServerError()
        }
    }
}
