use actix_web::{get, HttpResponse, Responder, web};

#[get("/hello-world")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/hello-valencia")]
async fn hello_valencia() -> impl Responder {
    HttpResponse::Ok().body("Hello Valencia")
}

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_world);
    cfg.service(hello_valencia);
}
