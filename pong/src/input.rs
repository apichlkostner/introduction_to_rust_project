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

    on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_UP, {
        world.move_sprite("player1", Pos { x: 0.0, y: -dist });
    });
    on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_DOWN, {
        world.move_sprite("player1", Pos { x: 0.0, y: dist });
    });
}
