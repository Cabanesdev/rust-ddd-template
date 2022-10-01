use actix_web::{get, post, web, HttpResponse, Responder};
use crate::contexts::gretting::shared::structs::{CityRequest, CityResponse};

#[get("/hello-world")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/hello-valencia")]
async fn hello_valencia() -> impl Responder {
    HttpResponse::Ok().body("Hello Valencia")
}

#[post("/gretting-from")]
async fn hello_from(city_info: web::Json<CityRequest>) -> impl Responder {
    let city = CityResponse {
        zip_code: city_info.zip_code.clone(),
        city: city_info.city.clone(),
        gretting: format!("Hello from {} with {} zip_code", city_info.city, city_info.zip_code),
    };
    return web::Json(city);
}

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_world);
    cfg.service(hello_valencia);
    cfg.service(hello_from);
}
