use crate::services::city_service;
use actix_web::{get, http::header::ContentType, HttpResponse, Responder};

#[get("/cities")]
async fn get_cities() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(city_service::get_cities_json())
}
