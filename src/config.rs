use figment::{
    Error, Figment,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GitlabPat {
    pub uri: String,
    pub token: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceConfig {
    #[serde(alias = "CACHE_TTL")]
    pub cache_ttl: u64,
    pub github_pat: String,
    pub gitlab_pats: Vec<GitlabPat>,
}

impl ServiceConfig {
    pub fn new() -> Result<Self, Error> {
        let config: ServiceConfig = Figment::new()
            .merge(Toml::file("config.toml"))
            .merge(Env::raw())
            .extract()?;
        Ok(config)
    }
}
