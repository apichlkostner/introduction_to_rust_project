//! Foreign Function Interface (FFI) bindings for the game engine.
//!
//! This module provides Rust wrappers for C functions and types used to interact
//! with the underlying game engine, including window management, sprite creation,
//! rendering, and input handling.

use std::ffi::CString;
use std::os::raw::{c_char, c_float, c_int};

/// Represents a sprite in the game engine.
///
/// # Fields
/// - `width`, `height`: Dimensions of the sprite.
/// - `color`: RGB color as an array of three integers.
/// - `x`, `y`: Position of the sprite.
#[repr(C)]
pub struct Sprite {
    pub width: c_int,
    pub height: c_int,
    pub color: [c_int; 3],
    pub x: c_float,
    pub y: c_float,
}

/// Opaque type representing a GLFW window.
#[repr(C)]
pub struct GLFWwindow {
    _private: [u8; 0], // opaque type
}

/// Key and action constants for input handling.
pub const GLFW_PRESS: c_int = 1;
pub const GLFW_KEY_SPACE: c_int = 32;
pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;

unsafe extern "C" {
    /// Creates a game window with the given title and dimensions.
    fn create_game_window(title: *const c_char, width: c_int, height: c_int);

    /// Creates a sprite at the given position, size, and color.
    fn create_sprite(
        x: c_float,
        y: c_float,
        width: c_int,
        height: c_int,
        r: c_int,
        g: c_int,
        b: c_int,
    ) -> *mut Sprite;

    /// Renders the given sprite.
    fn render_sprite(sprite: *mut Sprite);

    /// Updates the position of the given sprite.
    fn update_sprite_position(sprite: *mut Sprite, x: c_float, y: c_float);

    /// Updates the game window (swaps buffers, polls events).
    fn update_game_window();

    /// Clears the screen.
    fn clear_screen();

    /// Returns non-zero if the window should close.
    fn window_should_close() -> c_int;

    /// Gets the state of the specified key for the given window.
    fn get_key(window: *mut GLFWwindow, key: c_int) -> c_int;

    /// Returns a pointer to the main game window.
    fn get_window() -> *mut GLFWwindow;
}

/// Creates a game window with the specified title, width, and height.
///
/// # Arguments
/// * `title` - The window title.
/// * `width` - The window width in pixels.
/// * `height` - The window height in pixels.
pub fn rust_create_game_window(title: &str, width: i32, height: i32) {
    let t = CString::new(title).unwrap();
    unsafe {
        create_game_window(t.as_ptr(), width, height);
    }
}

/// Creates a sprite with the given position, size, and color.
///
/// # Arguments
/// * `x`, `y` - The position of the sprite.
/// * `width`, `height` - The size of the sprite.
/// * `r`, `g`, `b` - The RGB color values.
/// 
/// # Returns
/// Pointer to the created `Sprite`.
pub fn rust_create_sprite(x: f32, y: f32, width: i32, height: i32, r: i32, g: i32, b: i32) -> *mut Sprite{
    unsafe {
        return create_sprite(x, y, width, height, r, g, b);
    }
}

/// Renders the specified sprite.
///
/// # Arguments
/// * `sprite` - Pointer to the sprite to render.
pub fn rust_render_sprite(sprite: *mut Sprite) {
    unsafe {
        render_sprite(sprite);
    }
}

/// Updates the position of the specified sprite.
///
/// # Arguments
/// * `sprite` - Pointer to the sprite.
/// * `x`, `y` - New position.
pub fn rust_update_sprite_position(sprite: *mut Sprite, x: f32, y: f32) {
    unsafe {
        update_sprite_position(sprite, x, y);
    }
}

/// Updates the game window (swaps buffers, polls events).
pub fn rust_update_game_window() {
    unsafe {
        update_game_window();
    }
}

/// Clears the game window screen.
pub fn rust_clear_screen() {
    unsafe {
        clear_screen();
    }
}

/// Returns `true` if the window should close, `false` otherwise.
pub fn rust_window_should_close() -> bool {
    unsafe {
        window_should_close() != 0
    }
}

/// Gets the state of the specified key for the given window.
///
/// # Arguments
/// * `window` - Pointer to the window.
/// * `key` - Key code.
/// 
/// # Returns
/// Key state as integer.
pub fn rust_get_key(window: *mut GLFWwindow, key: i32) -> i32 {
    unsafe {
        get_key(window, key)
    }
}

/// Returns a pointer to the main game window.
pub fn rust_get_window() -> *mut GLFWwindow {
    unsafe {
        get_window()
    }
}