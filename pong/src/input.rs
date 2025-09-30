use crate::sprite::Pos;
use crate::world::World;
use game_engine::*;

/// Processes keyboard input and moves the player sprite accordingly.
///
/// # Arguments
///
/// * `world` - A mutable reference to the game world.
/// * `dt` - The delta time since the last frame, used to scale movement speed.
pub fn process(world: &mut World, dt: f32) {
    let speed = 0.3;
    let dist = speed * dt;
    let window_height = world.window.height;

    on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_UP, {
        let player = world.get_sprite("player1");
        let mut new_y = player.pos.y - dist;
        new_y = new_y.clamp(0.0, window_height - player.size.height);
        world.set_sprite_pos("player1", Pos { x: player.pos.x, y: new_y });
    });
    on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_DOWN, {
        let player = world.get_sprite("player1");
        let mut new_y = player.pos.y + dist;
        new_y = new_y.clamp(0.0, window_height - player.size.height);
        world.set_sprite_pos("player1", Pos { x: player.pos.x, y: new_y });
    });
}
