pub mod ffi;
#[macro_use]
pub mod macros;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    #[ignore]
    fn test_simple_game_loop() {
        start_window_and_game_loop!(|| {}, || {});
    }

    #[test]
    #[ignore]
    fn test_sprite_rendering() {
        start_window_and_game_loop!(
            || {
                spawn_sprite!(100.0, 100.0, 100, 100, 255, 0, 0);
            },
            || {}
        );
    }

    #[test]
    #[ignore]
    fn test_screen_clearing() {
        let sprite1 = ffi::rust_create_sprite(100.0, 100.0, 100, 100, 255, 0, 0);
        let sprite2 = ffi::rust_create_sprite(300.0, 300.0, 100, 100, 0, 0, 255);

        let start = Instant::now();
        let mut window_cleared = false;

        start_window_and_game_loop!(
            || {
                let elapsed = start.elapsed();
                let cond = elapsed < Duration::from_secs(2);

                if cond {
                    ffi::rust_render_sprite(sprite1);
                    tick!();
                } else {
                    if !window_cleared {
                        ffi::rust_clear_screen();
                        window_cleared = true;
                    }
                    ffi::rust_render_sprite(sprite2);
                    tick!();
                }
            },
            || {}
        );
    }

    #[test]
    #[ignore]
    fn test_key_presses() {
        ffi::rust_create_game_window("My game", 500, 500);
        let sprite1 = ffi::rust_create_sprite(100.0, 100.0, 100, 100, 255, 0, 0);
        let window = ffi::rust_get_window();

        let mut keys_pressed: [(i32, bool); 5] = [
            (ffi::GLFW_KEY_SPACE, false),
            (ffi::GLFW_KEY_RIGHT, false),
            (ffi::GLFW_KEY_LEFT, false),
            (ffi::GLFW_KEY_DOWN, false),
            (ffi::GLFW_KEY_UP, false),
        ];

        fn check_key_pressed(
            window: *mut ffi::GLFWwindow,
            keys_pressed: &mut [(i32, bool)],
        ) -> bool {
            let mut result = false;
            for (key, pressed) in keys_pressed.iter_mut() {
                on_key_press!(window, *key, || {
                    *pressed = true;
                    result = true;
                });
            }
            result
        }

        fn check_all_keys_pressed(keys_pressed: &[(i32, bool)]) -> bool {
            for &(_, pressed) in keys_pressed {
                if !pressed {
                    return false;
                }
            }
            true
        }

        while !ffi::rust_window_should_close() {
            let render = check_key_pressed(window, &mut keys_pressed);
            ffi::rust_clear_screen();
            if render {
                ffi::rust_render_sprite(sprite1);
            }
            tick!();

            if check_all_keys_pressed(&keys_pressed) {
                return;
            }
        }
    }

    #[test]
    #[ignore]
    fn test_sprite_position_update() {
        let mut x = -100.0;
        let mut y = -100.0;

        fn move_pos(x: f32, y: f32) -> (f32, f32) {
            (
                if x < 500.0 { x + 1.0 } else { -100.0 },
                if y < 500.0 { y + 1.0 } else { -100.0 },
            )
        }

        let sprite1 = spawn_sprite!(100.0, 100.0, 100, 100, 255, 0, 0);

        start_window_and_game_loop!("My game", 500, 500,
            || {
                (x, y) = move_pos(x, y);

                move_sprite!(sprite1, x, y, true);
            },
            || {}
        );
    }
}
