use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct SpriteData {
    pub width: i32,
    pub height: i32,
    pub x: f32,
    pub y: f32,
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl SpriteData {
    pub fn from_json_string(data: &str) -> Result<SpriteData> {
        let sprite_data: SpriteData = serde_json::from_str(data)?;
        Ok(sprite_data)
    }
}
