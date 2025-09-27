mod game;
mod sprite;
mod sprite_creator;
mod sprite_data;
mod world;

use game::*;
use game_engine::*;

use log::info;

fn main() {
    init_logger();
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

fn init_logger() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply()
        .unwrap();
}
