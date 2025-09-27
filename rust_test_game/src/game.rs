use crate::control;
use crate::sprite_creator;
use crate::sprite_data::SpriteData;
use crate::view;
use crate::world::World;
use crossbeam_channel::unbounded;
use game_engine::*;
use log::{error, info, warn};
use std::thread::{self, JoinHandle};
use std::time::Instant;

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
                        match &tx.send(sprite_data) {
                            Ok(_val) => {}
                            Err(err) => {
                                error!("Could not send sprite data {err}");
                            }
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
        self.world.set_player_sprite(100.0, 100.0, 100, 100, 255, 0, 0);
    }

    fn receive_new_sprites(&mut self) {
        match &self.rx {
            Some(rx) => {
                if let Ok(received) = rx.try_recv() {
                    self.world.add_sprite(
                        received.x,
                        received.y,
                        received.width,
                        received.height,
                        received.r,
                        received.g,
                        received.b,
                    );
                }
            }
            None => {
                warn!("No receiver active");
            }
        }
    }

    pub fn game_loop(&mut self) {
        rust_clear_screen();

        self.dt = self.last_time.elapsed().as_millis();
        if self.dt < 2 {
            self.dt = 2;
        }
        self.last_time = Instant::now();

        control::process_input(&mut self.world, self.dt as f32);

        self.receive_new_sprites();

        view::render(&self.world);
    }

    pub fn quit(self) -> bool {
        match &self.tx {
            Some(tx) => match &tx.send(()) {
                Ok(_) => {
                    info!("Send terminate message to child thread.");
                }
                Err(error) => {
                    error!("Could not send terminate message to child thread: {error}");
                }
            },
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
