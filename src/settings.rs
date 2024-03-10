use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub settings_version: String,
    pub max_embed_ratio: f32,
}