use crate::sprite::{Color, Pos, Size};
use crate::world::World;

pub fn calc_action(world: &mut World, dt: f32) {
    let myself = world.get_sprite("player2");
    let ball = world.get_sprite("ball");

    if ball.pos.y > myself.pos.y {
        world.move_sprite(
            "player2",
            Pos {
                x: 0.0,
                y: 0.1 * dt,
            },
        );
    } else if ball.pos.y < myself.pos.y {
        world.move_sprite(
            "player2",
            Pos {
                x: 0.0,
                y: -0.1 * dt,
            },
        );
    }
}
