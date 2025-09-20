#[macro_export]
macro_rules! spawn_sprite {
    ($x:expr, $y:expr, $width:expr, $height:expr, $r:expr, $g:expr, $b:expr) => {{
        let sprite = ffi::rust_create_sprite($x, $y, $width, $height, $r, $g, $b);
        ffi::rust_render_sprite(sprite);
        sprite
    }};
}

#[macro_export]
macro_rules! move_sprite {
    ($sprite:expr, $x:expr, $y:expr) => {
        let sprite = ffi::update_sprite_position($sprite, $x, $x);
        ffi::rust_render_sprite(sprite);
    };
    ($sprite:expr, $x:expr, $y:expr, $clear:expr) => {
        if $clear {
            ffi::rust_clear_screen();
        }
        ffi::rust_update_sprite_position($sprite, $x, $x);
        ffi::rust_render_sprite($sprite);
    };
}

#[macro_export]
macro_rules! tick {
    () => {
        ffi::rust_update_game_window();
        std::thread::sleep(Duration::from_millis(20));
    };
}

#[macro_export]
macro_rules! on_key_press {
    ($window:expr, $key:expr, $function:expr) => {
        if ffi::rust_get_key($window, $key) == ffi::GLFW_PRESS {
            $function()
        }
    };
}

#[macro_export]
macro_rules! start_window_and_game_loop {
    ($operation_start:expr, $operation_end:expr) => {
        ffi::rust_create_game_window("Test game", 800, 600);
        while !ffi::rust_window_should_close() {
            $operation_start();
            $operation_end();
            tick!();
        }
    };
    ($game_name:expr, $width:expr, $height:expr, $operation_start:expr, $operation_end:expr) => {
        ffi::rust_create_game_window($game_name, $width, $height);
        while !ffi::rust_window_should_close() {
            $operation_start();            
            tick!();
            $operation_end();
        }
    };
}
