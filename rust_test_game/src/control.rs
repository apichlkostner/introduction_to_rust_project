use game_engine::*;
use crate::world::World;
use log::{info};

pub fn process_input(world: &mut World, dt: f32) {
        let sprite = &mut world.player_sprite;
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