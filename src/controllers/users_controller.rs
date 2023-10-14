use crate::services::user_service;
use actix_web::{get, http::header::ContentType, HttpResponse, Responder};

#[get("/users")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(user_service::get_user_json())
}
