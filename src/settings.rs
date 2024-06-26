use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub settings_version: String,
    pub max_embed_ratio: f32,
    pub min_capacity: u64,
    pub prog_code: String,
    pub byte_chunk: u32,
    pub secret_folder: String,
    pub thumb_folder: String,
    pub window_width: i32,
    pub window_height: i32,
}
