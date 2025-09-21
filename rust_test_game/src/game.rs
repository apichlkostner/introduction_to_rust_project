use crate::sprite_creator;
use crate::sprite_data::SpriteData;
use game_engine::*;
use log::{info, warn};
use std::sync::mpsc;
use std::thread;

pub struct Sprite {
    c_sprite: *mut ffi::Sprite,
}

pub struct Game {
    sprites: Vec<Sprite>,
    rx: Option<mpsc::Receiver<SpriteData>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
            rx: None,
        }
    }

    pub fn init(&mut self) {
        info!("Init game threads");
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);

        thread::spawn(move || {
            loop {
                let sprite_data_result = sprite_creator::get_new_sprite_data();
                match sprite_data_result {
                    Ok(sprite_data) => {
                        info!("Sprite successfully created");
                        tx.send(sprite_data).unwrap();
                    }
                    Err(err) => {
                        warn!("Error loading new sprite_data: {err}");
                    }
                }
            }
        });
    }

    fn create_sprite_ptr(&self, sprite_data: &SpriteData) -> *mut ffi::Sprite {
        spawn_sprite!(
            sprite_data.x,
            sprite_data.y,
            sprite_data.width,
            sprite_data.height,
            sprite_data.r,
            sprite_data.b,
            sprite_data.b
        )
    }

    pub fn game_loop(&mut self) {
        rust_clear_screen();

        match &self.rx {
            Some(rx) => {
                if let Ok(received) = rx.try_recv() {
                    let sprite_ptr = self.create_sprite_ptr(&received);
                    let sprite = Sprite {
                        c_sprite: sprite_ptr,
                    };
                    self.sprites.push(sprite);
                }
            }
            None => {
                warn!("No receiver active");
            }
        }

        for sprite_ref in &self.sprites {
            rust_render_sprite(sprite_ref.c_sprite);
        }
    }

    pub fn quit(&mut self) -> bool {
        true
    }
}
