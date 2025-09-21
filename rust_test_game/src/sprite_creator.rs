use crate::sprite_data::SpriteData;
use reqwest;

pub fn get_new_sprite_data() -> Result<SpriteData, String> {
    let body = reqwest::blocking::get(
        "https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler",
    );

    match body {
        Ok(text) => {
            let pt = text.text();
            match pt {
                Ok(json_string) => {
                    let sd = SpriteData::from_json_string(&json_string);
                    match sd {
                        Ok(sprite_data) => Ok(sprite_data),
                        Err(_) => Err(String::from("Error parsing sprite data"))
                    }
                }
                Err(_) => Err(String::from("Error parsing json string")),
            }
        }
        Err(_) => Err(String::from("Error downloading json string")),
    }
}

#[cfg(test)]
mod tests {
    use crate::sprite_creator;

    #[test]
    #[ignore]
    fn create_sprite() {
        let sprite_data = sprite_creator::get_new_sprite_data();

        assert!(sprite_data.is_ok(), "Sprite creation should be successful");
        let sprite = sprite_data.unwrap();
        assert!(sprite.width < 1024);
    }
}
