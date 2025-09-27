use crate::sprite_creator;
use crate::sprite_data::SpriteData;
use crate::sprite::Sprite;
use crate::world::World;
use crossbeam_channel::{unbounded};
use game_engine::*;
use log::{info, warn, error};
use std::thread::{self, JoinHandle};
use std::time::{Instant};



pub struct Game {
    world: World,
    last_time: Instant,
    dt: u128,
    rx: Option<crossbeam_channel::Receiver<SpriteData>>,
    tx: Option<crossbeam_channel::Sender<()>>,
    handles: Vec<JoinHandle<()>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            world: World::empty(),
            last_time: Instant::now(),
            dt: 20,
            rx: None,
            tx: None,
            handles: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        info!("Init game threads");

        let (tx_term, rx_term) = unbounded::<()>();
        self.tx = Some(tx_term);

        let (tx, rx) = unbounded();
        self.rx = Some(rx);

        let handle = thread::spawn(move || {
            loop {
                if let Ok(_received) = rx_term.try_recv() {
                    return;
                }
                let sprite_data_result = sprite_creator::get_new_sprite_data();
                match sprite_data_result {
                    Ok(sprite_data) => {
                        info!("Sprite successfully created");
                        match &tx.send(sprite_data){
                            Ok(_val) => {},
                            Err(err) => {error!("Could not send sprite data {err}");}
                        }
                    }
                    Err(err) => {
                        warn!("Error loading new sprite_data: {err}");
                    }
                }
            }
        });
        self.handles.push(handle);

        self.last_time = Instant::now();
        let sprite = Sprite::new(100.0, 100.0, 100, 100, 255, 0, 0);
        self.world.player_sprite = sprite;
    }


    fn process_input(&mut self) {
        let sprite = &mut self.world.player_sprite;
        let dt = self.dt as f32;
        let speed = 0.1;
        let delta_pos = speed * dt;

        on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_RIGHT, {
            sprite.x += delta_pos;
            let x = sprite.x;
            let y = sprite.y;
            info!("Move down {delta_pos} pixels to ({x}, {y})");
            move_sprite!(sprite.get_c_sprite(), sprite.x, sprite.y);
        });
        on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_LEFT, {
            sprite.x -= delta_pos;
            let x = sprite.x;
            let y = sprite.y;
            info!("Move down {delta_pos} pixels to ({x}, {y})");
            move_sprite!(sprite.get_c_sprite(), sprite.x, sprite.y);
        });
        on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_UP, {
            sprite.y -= delta_pos;
            let x = sprite.x;
            let y = sprite.y;
            info!("Move down {delta_pos} pixels to ({x}, {y})");
            move_sprite!(sprite.get_c_sprite(), sprite.x, sprite.y);
        });
        on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_DOWN, {
            sprite.y += delta_pos;
            let x = sprite.x;
            let y = sprite.y;
            info!("Move down {delta_pos} pixels to ({x}, {y})");
            move_sprite!(sprite.get_c_sprite(), sprite.x, sprite.y);
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
                    let sprite = Sprite::new(
                        received.x,
                        received.y,
                        received.width,
                        received.height,
                        received.r,
                        received.g,
                        received.b,
                    );
                    self.world.sprites.push(sprite);
                }
            }
            None => {
                warn!("No receiver active");
            }
        }

        rust_render_sprite(self.world.player_sprite.get_c_sprite());
        for sprite_ref in &self.world.sprites {
            rust_render_sprite(sprite_ref.get_c_sprite());
        }
    }

    pub fn quit(self) -> bool {
        

        match &self.tx {
            Some(tx) => {
                match &tx.send(()) {
                    Ok(_) => {info!("Send terminate message to child thread.");}
                    Err(error) => {error!("Could not send terminate message to child thread: {error}");}
                }},
            None => {
                warn!("No sender active");
            }
        }

        for handle in self.handles {
            handle.join().expect("Thread panicked");
        }
        true
    }
}
