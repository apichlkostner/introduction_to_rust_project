use game_engine::*;
use crate::world::World;

pub fn process_input(world: &mut World, dt: f32) {
        let speed = 0.1;
        let dist = speed * dt;

        on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_RIGHT, {
            world.move_player(dist, 0.0);
        });
        on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_LEFT, {
            world.move_player(-dist, 0.0);
        });
        on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_UP, {
            world.move_player(0.0, -dist);
        });
        on_key_press!(ffi::rust_get_window(), ffi::GLFW_KEY_DOWN, {
            world.move_player(0.0, dist);
        });
    }