use crate::sprite::Pos;
use crate::world::World;

pub fn calc_action(world: &mut World, dt: f32) {
    let myself = world.get_sprite("player2");
    let ball = world.get_sprite("ball");
    let speed = 0.3;
    let middle_pos = myself.pos.y + myself.size.height / 2.0;
    let middle_ball_pos = ball.pos.y + ball.size.height / 2.0;
    let delta = speed * dt;

    if (middle_ball_pos - middle_pos) > 2.0 * delta {
        world.move_sprite("player2", Pos { x: 0.0, y: delta });
    } else if (middle_ball_pos - middle_pos) < -2.0 * delta {
        world.move_sprite("player2", Pos { x: 0.0, y: -delta });
    }
}
