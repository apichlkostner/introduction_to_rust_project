mod control;
mod game;
mod logging;
mod sprite;
mod sprite_creator;
mod sprite_data;
mod view;
mod world;

use game::*;
use game_engine::*;

use log::info;

fn main() {
    logging::init_logger();
    info!("Starting rust_test_game");

    let mut game = Game::new();

    start_window_and_game_loop!(
        "Test game 1",
        1024,
        768,
        {
            game.init();
        },
        {
            game.game_loop();
        },
        {
            game.quit();
        }
    );
}

