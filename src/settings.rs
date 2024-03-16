use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub settings_version: String,
    pub max_embed_ratio: f32,
    pub min_capacity: u32,
    pub prog_code: String,
}