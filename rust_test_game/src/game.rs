use crate::sprite_creator;
use crate::sprite_data::SpriteData;
use crossbeam_channel::{select, unbounded};
use game_engine::*;
use log::{info, warn};
use std::thread;
use std::time::{Duration, Instant};

pub struct Sprite {
    c_sprite: *mut ffi::Sprite,
    x: f32,
    y: f32,
}

pub struct Game {
    sprites: Vec<Sprite>,
    last_time: Instant,
    dt: u128,
    rx: Option<crossbeam_channel::Receiver<SpriteData>>,
    tx: Option<crossbeam_channel::Sender<()>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
            last_time: Instant::now(),
            dt: 20,
            rx: None,
            tx: None,
        }
    }

    pub fn init(&mut self) {
        info!("Init game threads");

        let (tx_term, rx_term) = unbounded::<()>();
        self.tx = Some(tx_term);

        let (tx, rx) = unbounded();
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
        self.last_time = Instant::now();

        let sprite_ptr = spawn_sprite!(100.0, 100.0, 100, 100, 255, 0, 0);
        let sprite = Sprite {
            c_sprite: sprite_ptr,
            x: 100.0,
            y: 100.0,
        };
        self.sprites.push(sprite);
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

    fn process_input(&mut self) {
        let sprite = &mut self.sprites[0];
        let dt = self.dt as f32;
        let speed = 0.1;
        let delta_pos = speed * dt;

        on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_RIGHT, {
            sprite.x += delta_pos;
            let x = sprite.x;
            let y = sprite.y;
            info!("Move down {delta_pos} pixels to ({x}, {y})");
            move_sprite!(sprite.c_sprite, sprite.x, sprite.y);
        });
        on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_LEFT, {
            sprite.x -= delta_pos;
            let x = sprite.x;
            let y = sprite.y;
            info!("Move down {delta_pos} pixels to ({x}, {y})");
            move_sprite!(sprite.c_sprite, sprite.x, sprite.y);
        });
        on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_UP, {
            sprite.y -= delta_pos;
            let x = sprite.x;
            let y = sprite.y;
            info!("Move down {delta_pos} pixels to ({x}, {y})");
            move_sprite!(sprite.c_sprite, sprite.x, sprite.y);
        });
        on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_DOWN, {
            sprite.y += delta_pos;
            let x = sprite.x;
            let y = sprite.y;
            info!("Move down {delta_pos} pixels to ({x}, {y})");
            move_sprite!(sprite.c_sprite, sprite.x, sprite.y);
        });
    }

    pub fn game_loop(&mut self) {
        rust_clear_screen();

        self.dt = self.last_time.elapsed().as_millis();
        // let dt = self.dt;
        // info!("dt = {dt}");
        if self.dt < 2 {
            self.dt = 2;
        }
        self.last_time = Instant::now();

        self.process_input();

        match &self.rx {
            Some(rx) => {
                if let Ok(received) = rx.try_recv() {
                    let sprite_ptr = self.create_sprite_ptr(&received);
                    let sprite = Sprite {
                        c_sprite: sprite_ptr,
                        x: received.x,
                        y: received.y,
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
