use actix_web::{HttpResponse, Responder, get};

#[get("/activity")]
pub async fn handle_request() -> impl Responder {
    // Your implementation here
    HttpResponse::Ok().body("Hello, World!")
}
