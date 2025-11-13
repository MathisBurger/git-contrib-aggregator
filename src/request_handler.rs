use actix_web::{HttpResponse, Responder, get, web::Data};

use crate::config::ServiceConfig;

#[get("/api/activity")]
pub async fn handle_request(config: Data<ServiceConfig>) -> impl Responder {
    // TODO: implement github activity retrival
    // TODO: implement gitlab activity retrival
    // TODO: Add caching logic here
    HttpResponse::Ok().body("Hello, World!")
}
