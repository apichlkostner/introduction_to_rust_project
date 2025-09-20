//use core::time;
//use std::thread;

use std::slice::Windows;

mod ffi;

fn main() {
    ffi::rust_create_game_window("My game", 500, 500);
    let sprite1 = ffi::rust_create_sprite(100.0, 100.0, 100, 100, 255, 0, 0);
    let sprite2 = ffi::rust_create_sprite(300.0, 300.0, 100, 100, 0, 0, 255);

    let mut x = -100.0;
    let mut y = -100.0;
    let window = ffi::rust_get_window();
    let mut render_second = false;

    while !ffi::rust_window_should_close() {
        (x, y) = move_pos(x, y);
        render_second = second_sprite_render(window);

        ffi::rust_update_sprite_position(sprite1, x, y);
        ffi::rust_clear_screen();
        ffi::rust_render_sprite(sprite1);
        if (render_second) {
            ffi::rust_render_sprite(sprite2);
        }
        ffi::rust_update_game_window();
    }
}

fn move_pos(x: f32, y: f32) -> (f32, f32) {
    (
        if x < 500.0 { x + 1.0 } else { -100.0 },
        if y < 500.0 { y + 1.0 } else { -100.0 },
    )
}

fn second_sprite_render(window: *mut ffi::GLFWwindow) -> bool {
    let current_key_state = ffi::rust_get_key(window, ffi::GLFW_KEY_SPACE);

    current_key_state == 1
}
