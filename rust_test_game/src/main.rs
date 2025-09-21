mod game;
mod sprite_creator;
mod sprite_data;

use game::*;
use game_engine::*;

fn main() {
    let mut game = Game::new();

    start_window_and_game_loop!(
        "Test game 1",
        500,
        500,
        {
            game.game_loop_start();
        },
        {
            let run = game.game_loop_end();
            if !run {
                break;
            }
        }
    );
}
