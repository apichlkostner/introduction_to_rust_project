//! AI player logic for Pong.
//!
//! This module implements the logic for controlling the AI paddle ("player2").
//! The AI paddle attempts to follow the ball's vertical position with a fixed speed.

use crate::sprite::Pos;
use crate::world::World;

/// Calculates and applies the next action for the AI player ("player2").
///
/// Moves the AI paddle up or down to track the ball's vertical position,
/// using a fixed speed. The movement is only triggered if the ball is
/// sufficiently far from the paddle's center to avoid jittery movement.
///
/// # Arguments
///
/// * `world` - Mutable reference to the game world containing all sprites.
/// * `dt` - The time delta since the last update (in seconds).
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
