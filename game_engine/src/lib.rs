pub mod ffi;
pub use ffi::*;
#[macro_use]
pub mod macros;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_simple_game_loop() {
        start_window_and_game_loop!({}, {}, {});
    }

    #[test]
    #[ignore]
    fn test_sprite_rendering() {
        start_window_and_game_loop!(
            {},
            {
                spawn_sprite!(100.0, 100.0, 100, 100, 255, 0, 0);
            },
            {}
        );
    }

    #[test]
    #[ignore]
    fn test_screen_clearing() {
        let sprite1 = ffi::rust_create_sprite(100.0, 100.0, 100, 100, 255, 0, 0);
        let sprite2 = ffi::rust_create_sprite(300.0, 300.0, 100, 100, 0, 0, 255);

        let start = std::time::Instant::now();
        let mut window_cleared = false;

        start_window_and_game_loop!(
            {},
            {
                let elapsed = start.elapsed();
                let cond = elapsed < std::time::Duration::from_secs(2);

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
            {}
        );
    }

    #[test]
    #[ignore]
    fn test_key_presses() {
        struct Game {
            sprite: *mut ffi::Sprite,
            keys_pressed: [(i32, bool); 5],
        }

        impl Game {
            fn new() -> Self {
                Self {
                    sprite: ffi::rust_create_sprite(100.0, 100.0, 100, 100, 255, 0, 0),
                    keys_pressed: [
                        (ffi::GLFW_KEY_SPACE, false),
                        (ffi::GLFW_KEY_RIGHT, false),
                        (ffi::GLFW_KEY_LEFT, false),
                        (ffi::GLFW_KEY_DOWN, false),
                        (ffi::GLFW_KEY_UP, false),
                    ],
                }
            }

            fn check_key_pressed(&mut self) -> bool {
                let mut result = false;
                for (key, pressed) in self.keys_pressed.iter_mut() {
                    on_key_press!(ffi::rust_get_window(), *key, {
                        *pressed = true;
                        result = true;
                    });
                }
                result
            }

            fn check_all_keys_pressed(&self) -> bool {
                for (_, pressed) in self.keys_pressed {
                    if !pressed {
                        return false;
                    }
                }
                true
            }

            fn game_loop_start(&mut self) {
                let render = self.check_key_pressed();
                ffi::rust_clear_screen();
                if render {
                    ffi::rust_render_sprite(self.sprite);
                }
            }

            fn game_loop_end(&mut self) -> bool {
                if self.check_all_keys_pressed() {
                    return false;
                }
                true
            }
        }

        let mut game = Game::new();

        start_window_and_game_loop!(
            {},
            {
                game.game_loop_start();
                let run = game.game_loop_end();
                if !run {
                    break;
                }
            },
            {}
        );
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

        start_window_and_game_loop!(
            "My game",
            500,
            500,
            {},
            {
                (x, y) = move_pos(x, y);

                move_sprite!(sprite1, x, y, true);
            },
            {}
        );
    }
}
