use actix_web::{
    web::{self, Form},
    HttpResponse,
};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// Let's start simple: we always return a 200 OK
pub async fn subscribe(form: Form<FormData>, connection: web::Data<PgPool>) -> HttpResponse {
}
