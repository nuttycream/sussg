use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub general: GeneralConfig,
}

#[derive(Deserialize, Serialize)]
pub struct GeneralConfig {
    pub url: String,
    pub output_dir: String,
}
