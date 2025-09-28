use crate::sprite::{Color, Pos, Size};
use crate::world::World;

pub struct Ball {
    pub direction: i32,
}

impl Ball {
    pub fn calc_action(&mut self, world: &mut World, dt: f32) {
        let ball = world.get_sprite("ball");

        if self.direction == 0 {
            if ball.pos.y > 0.0 {
                world.move_sprite(
                    "ball",
                    Pos {
                        x: 0.0,
                        y: -0.1 * dt,
                    },
                );
            } else {
                self.direction = 1;
            }
        } else {
            if ball.pos.y < world.window.height as f32 {
                world.move_sprite(
                    "ball",
                    Pos {
                        x: 0.0,
                        y: 0.1 * dt,
                    },
                );
            } else {
                self.direction = 0;
            }
        }
    }
}
