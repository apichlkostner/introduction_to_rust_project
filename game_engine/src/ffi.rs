use std::ffi::CString;
use std::os::raw::{c_char, c_float, c_int};

#[repr(C)]
pub struct Sprite {
    pub width: c_int,
    pub height: c_int,
    pub color: [c_int; 3],
    pub x: c_float,
    pub y: c_float,
}

#[repr(C)]
pub struct GLFWwindow {
    _private: [u8; 0], // opaque type
}

pub const GLFW_PRESS: c_int = 1;
pub const GLFW_KEY_SPACE: c_int = 32;
pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;

unsafe extern "C" {
    fn create_game_window(title: *const c_char, width: c_int, height: c_int);

    fn create_sprite(
        x: c_float,
        y: c_float,
        width: c_int,
        height: c_int,
        r: c_int,
        g: c_int,
        b: c_int,
    ) -> *mut Sprite;

    fn render_sprite(sprite: *mut Sprite);

    fn update_sprite_position(sprite: *mut Sprite, x: c_float, y: c_float);

    fn update_game_window();

    fn clear_screen();

    fn window_should_close() -> c_int;

    fn get_key(window: *mut GLFWwindow, key: c_int) -> c_int;

    fn get_window() -> *mut GLFWwindow;
}

pub fn rust_create_game_window(title: &str, width: i32, height: i32) {
    let t = CString::new(title).unwrap();
    unsafe {
        create_game_window(t.as_ptr(), width, height);
    }
}

pub fn rust_create_sprite(x: f32, y: f32, width: i32, height: i32, r: i32, g: i32, b: i32) -> *mut Sprite{
    unsafe {
        return create_sprite(x, y, width, height, r, g, b);
    }
}

pub fn rust_render_sprite(sprite: *mut Sprite) {
    unsafe {
        render_sprite(sprite);
    }
}

pub fn rust_update_sprite_position(sprite: *mut Sprite, x: f32, y: f32) {
    unsafe {
        update_sprite_position(sprite, x, y);
    }
}

pub fn rust_update_game_window() {
    unsafe {
        update_game_window();
    }
}

pub fn rust_clear_screen() {
    unsafe {
        clear_screen();
    }
}

pub fn rust_window_should_close() -> bool {
    unsafe {
        window_should_close() != 0
    }
}

pub fn rust_get_key(window: *mut GLFWwindow, key: i32) -> i32 {
    unsafe {
        get_key(window, key)
    }
}

pub fn rust_get_window() -> *mut GLFWwindow {
    unsafe {
        get_window()
    }
}