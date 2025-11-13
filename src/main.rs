use crate::config::ServiceConfig;
use actix_web::{App, HttpServer, web::Data};
use figment::error::Kind::MissingField;
use std::io::Result;

mod config;
mod data_model;
mod github;
mod gitlab;
mod request_handler;

#[actix_web::main]
async fn main() -> Result<()> {
    let config: ServiceConfig = match ServiceConfig::new() {
        Ok(config) => config,
        Err(error) => {
            println!("Couldn't read config");
            if let MissingField(f) = error.kind {
                println!("Missing field: '{}'", f.to_uppercase());
            } else {
                println!("Error: {:?}", error);
                std::process::exit(2)
            }
            std::process::exit(1)
        }
    };

    let app_state = Data::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(request_handler::handle_request)
    })
    .bind("0.0.0.0:8080")
    .expect("Already in use")
    .run()
    .await
}
