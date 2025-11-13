use actix_web::{HttpResponse, Responder, get};
use chrono::{DateTime, Duration, Utc};
use std::sync::{Arc, RwLock};

use crate::{
    config::ServiceConfig, data_model::ContributionsResponse, github::fetch_github_contributions,
    gitlab::fetch_gitlab_contributions,
};

struct ContributionsCache {
    data: ContributionsResponse,
    last_update: DateTime<Utc>,
}

// Global cache
static GLOBAL_CACHE: once_cell::sync::Lazy<Arc<RwLock<Option<ContributionsCache>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(None)));

#[get("/api/activity")]
pub async fn handle_request(config: actix_web::web::Data<ServiceConfig>) -> impl Responder {
    {
        let read_guard = GLOBAL_CACHE.read().unwrap();
        if let Some(cached) = &*read_guard {
            if Utc::now() - cached.last_update < Duration::hours(config.cache_ttl as i64) {
                return HttpResponse::Ok().json(&cached.data);
            }
        }
    }

    let mut write_guard = GLOBAL_CACHE.write().unwrap();

    let mut response = ContributionsResponse::default();
    fetch_github_contributions(&config.github_pat, &mut response).await;
    for gitlabpat in &config.gitlab_pats {
        fetch_gitlab_contributions(gitlabpat, &mut response).await;
    }

    *write_guard = Some(ContributionsCache {
        data: response.clone(),
        last_update: Utc::now(),
    });

    HttpResponse::Ok().json(response)
}
