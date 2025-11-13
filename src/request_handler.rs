use actix_web::{HttpResponse, Responder, get, web::Data};

use crate::{
    config::ServiceConfig, data_model::ContributionsResponse, github::fetch_github_contributions,
};

#[get("/api/activity")]
pub async fn handle_request(config: Data<ServiceConfig>) -> impl Responder {
    let mut response = ContributionsResponse::default();

    fetch_github_contributions(&config.github_pat, &mut response).await;

    // TODO: implement github activity retrival
    // TODO: implement gitlab activity retrival
    // TODO: Add caching logic here
    HttpResponse::Ok().json(response)
}
