//! Macros for simplifying game engine operations.
//!
//! This module provides macros for spawning and moving sprites, handling game loop
//! logic, processing key presses, and managing window updates.

/// Spawns a sprite at the given position, size, and color, renders it, and returns a pointer to it.
///
/// # Example
/// ```ignore
/// let sprite = spawn_sprite!(100.0, 100.0, 50, 50, 255, 0, 0);
/// ```
#[macro_export]
macro_rules! spawn_sprite {
    ($x:expr, $y:expr, $width:expr, $height:expr, $r:expr, $g:expr, $b:expr) => {{
        let sprite = ffi::rust_create_sprite($x, $y, $width, $height, $r, $g, $b);
        ffi::rust_render_sprite(sprite);
        sprite
    }};
}

/// Moves a sprite to a new position and renders it. Optionally clears the screen before moving.
///
/// # Example
/// ```ignore
/// move_sprite!(sprite, 200.0, 200.0);
/// move_sprite!(sprite, 200.0, 200.0, true);
/// ```
#[macro_export]
macro_rules! move_sprite {
    ($sprite:expr, $x:expr, $y:expr) => {
        ffi::rust_update_sprite_position($sprite, $x, $y);
        ffi::rust_render_sprite($sprite);
    };
    ($sprite:expr, $x:expr, $y:expr, $clear:expr) => {
        if $clear {
            ffi::rust_clear_screen();
        }
        ffi::rust_update_sprite_position($sprite, $x, $y);
        ffi::rust_render_sprite($sprite);
    };
}

/// Advances the game loop by updating the window and sleeping for ~16ms (60 FPS).
///
/// # Example
/// ```ignore
/// tick!();
/// ```
#[macro_export]
macro_rules! tick {
    () => {
        ffi::rust_update_game_window();
        std::thread::sleep(std::time::Duration::from_millis(16));
    };
}

/// Executes a function if the specified key is pressed in the given window.
///
/// # Example
/// ```ignore
/// on_key_press!(window, ffi::GLFW_KEY_SPACE, || { println!("Space pressed!"); });
/// ```
#[macro_export]
macro_rules! on_key_press {
    ($window:expr, $key:expr, $function:expr) => {
        if ffi::rust_get_key($window, $key) == ffi::GLFW_PRESS {
            $function()
        }
    };
}

/// Starts the game window and runs the main game loop, with optional initialization and cleanup blocks.
///
/// # Example
/// ```ignore
/// start_window_and_game_loop!(
///     "My Game", 800, 600,
///     { /* init code */ },
///     { /* per-frame code */ },
///     { /* cleanup code */ }
/// );
/// ```
///
/// If no arguments are given, defaults to a test window.
/// ```
/// start_window_and_game_loop!({},{},{});
/// ```
#[macro_export]
macro_rules! start_window_and_game_loop {
    ($game_name:expr, $width:expr, $height:expr, $init:block, $enter:block, $exit:block) => {
        ffi::rust_create_game_window($game_name, $width, $height);

        $init
        while !ffi::rust_window_should_close() {
            $enter
            tick!();
        }
        $exit
    };
    ($init:block, $enter:block, $exit:block) => {
        ffi::rust_create_game_window("test game 1", 1024, 768);

        $init
        while !ffi::rust_window_should_close() {
            $enter
            tick!();
        }
        $exit
    };
}
