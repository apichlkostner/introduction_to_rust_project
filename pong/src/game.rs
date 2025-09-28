use crate::ai_player;
use crate::ball::Ball;
use crate::input;
use crate::sprite::{Pos, Size, Color};
use crate::view;
use crate::world::World;
use game_engine::*;
use log::{error, info, warn};
use std::time::Instant;

/// Main game structure holding the world, timing, communication channels, and thread handles.
pub struct Game {
    world: World,
    ball: Ball,
    last_time: Instant,
    num_sprites: i32,
}

impl Game {
    /// Creates a new `Game` instance with an empty world and initializes timing and channels.
    pub fn new() -> Self {
        Self {
            world: World::empty(),
            ball: Ball{direction: 0},
            last_time: Instant::now(),
            num_sprites: 0,
        }
    }

    /// Initializes the game, sets up threads for sprite creation, and sets the initial player sprite.
    pub fn init(&mut self) {
        info!("Init game threads");

        self.last_time = Instant::now();

        // ✅ Updated: use Pos, Size, Color
        self.world.add_sprite("player1",
            Pos { x: 20.0, y: 100.0 },
            Size { width: 30, height: 200 },
            Color { r: 255, g: 255, b: 255 },
        );
        self.world.add_sprite("player2",
            Pos { x: 1024.0 - 50.0, y: 200.0 },
            Size { width: 30, height: 200 },
            Color { r: 255, g: 255, b: 255 },
        );

        self.world.add_sprite("ball",
            Pos { x: 1024.0 / 2.0 - 30.0, y: 50.0 },
            Size { width: 30, height: 30 },
            Color { r: 255, g: 255, b: 255 },
        );
    }


    /// Calculates the delta time (dt) since the last frame in milliseconds.
    fn calc_dt(&mut self) -> f32 {
        let dt = self.last_time.elapsed().as_millis();

        self.last_time = Instant::now();

        if dt > 2 { dt as f32 } else { 2.0 }
    }

    /// Main game loop: clears the screen, processes input, receives new sprites, and renders the world.
    pub fn game_loop(&mut self) {
        rust_clear_screen();

        let dt = self.calc_dt();

        input::process(&mut self.world, dt);
        ai_player::calc_action(&mut self.world, dt);
        self.ball.calc_action(&mut self.world, dt);

        view::render(&self.world);
    }

    /// Cleans up the game, sends termination signals to threads, and waits for them to finish.
    ///
    /// # Returns
    ///
    /// * `true` if cleanup was successful.
    pub fn quit(self) -> bool {
        true
    }
}
