use crate::app::server::application::routes::configure_routes;

use actix_web::{App, HttpServer};

/// This function will create an Actix server
#[actix_web::main]
pub async fn create_and_start() -> std::io::Result<()> {
    println!("Server started in port: 3000");
    HttpServer::new(move || App::new().configure(configure_routes))
        .bind(("0.0.0.0", 3000))
        .unwrap()
        .run()
        .await
}
