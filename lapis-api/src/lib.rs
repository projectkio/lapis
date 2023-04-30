#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

pub struct ApiStartupParams {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expired_in: String,
    pub jwt_max_age: i32,
}

#[get("/api/v1/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "pong!"
    }))
}

#[actix_web::main]
pub async fn init(params: ApiStartupParams) -> std::io::Result<()> {
    info!("API starting...");

    HttpServer::new(move || App::new().wrap(Logger::default()).service(health_check))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
