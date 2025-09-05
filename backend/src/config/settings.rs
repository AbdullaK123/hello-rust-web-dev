use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub database_url: String,
    #[serde(default = "default_max_pool_size")]
    pub max_pool_size: u32,
    #[serde(default = "default_min_idle_size")]
    pub min_idle_size: u32
}

fn default_max_pool_size() -> u32 {10}

fn default_min_idle_size() -> u32 {2}


pub fn get_settings() -> Result<Settings, serde_env::Error> {
    dotenvy::dotenv().ok();
    serde_env::from_env()
}