use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub settings_version: String,
    pub max_embed_ratio: f32,
    pub min_capacity: u64,
    pub prog_code: String,
    pub pw_hash_len: u8,
    pub byte_chunk: u32,
}
