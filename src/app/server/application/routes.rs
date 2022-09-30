use actix_web::web;
use crate::contexts::blog::infrastructure::controllers::grettings;


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/gretting").configure(grettings::router));
}
