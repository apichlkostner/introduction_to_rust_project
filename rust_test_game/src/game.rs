use crate::sprite_creator;
use crate::sprite_data::SpriteData;
use game_engine::*;

pub struct Pos {
    x: f32,
    y: f32,
}

pub struct Sprite {
    c_sprite: *mut ffi::Sprite,
    //pos: Pos,
}

pub struct Game {
    sprites: Vec<Sprite>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
        }
    }

    fn spawn_new_sprite(&self) -> Result<*mut ffi::Sprite, String> {
        let sprite_data_result = sprite_creator::get_new_sprite_data();
        match sprite_data_result {
            Ok(sprite_data) => Ok(spawn_sprite!(
                sprite_data.x,
                sprite_data.y,
                sprite_data.width,
                sprite_data.height,
                sprite_data.r,
                sprite_data.b,
                sprite_data.b
            )),
            Err(_) => Err(String::from("Error creating new sprite")),
        }
    }

    pub fn game_loop_start(&mut self) {
        rust_clear_screen();
        let new_sprite = self.spawn_new_sprite();
        match new_sprite {
            Ok(sprite) => {
                let sprite = Sprite { c_sprite: sprite };
                self.sprites.push(sprite);
            }
            Err(_) => {
                println!("Error spawning sprite");
            }
        }
        for sprite_ref in &self.sprites {
            rust_render_sprite(sprite_ref.c_sprite);
        }
    }

    pub fn game_loop_end(&mut self) -> bool {
        true
    }
}
