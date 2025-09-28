use serde::{Deserialize, Serialize};
use serde_json::Result;

/// Serializable data structure representing the properties of a sprite.
#[derive(Serialize, Deserialize)]
pub struct SpriteData {
    /// The width of the sprite.
    pub width: i32,
    /// The height of the sprite.
    pub height: i32,
    /// The x position of the sprite.
    pub x: f32,
    /// The y position of the sprite.
    pub y: f32,
    /// The red color component.
    pub r: i32,
    /// The green color component.
    pub g: i32,
    /// The blue color component.
    pub b: i32,
}

impl SpriteData {
    /// Creates a `SpriteData` instance from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `data` - A JSON string representing a sprite's data.
    ///
    /// # Returns
    ///
    /// * `Result<SpriteData>` - The deserialized `SpriteData` or an error.
    pub fn from_json_string(data: &str) -> Result<SpriteData> {
        let sprite_data: SpriteData = serde_json::from_str(data)?;
        Ok(sprite_data)
    }
}
