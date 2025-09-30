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
    let window_height = world.window.height;
    let mut new_y = myself.pos.y;

    if (middle_ball_pos - middle_pos).abs() > 2.0 * delta {
        if middle_ball_pos > middle_pos {
            new_y += delta;
        } else {
            new_y -= delta;
        }
        // Clamp the new_y so the paddle stays within the window
        new_y = new_y.clamp(0.0, window_height - myself.size.height);
        world.set_sprite_pos("player2", Pos { x: myself.pos.x, y: new_y });
    }
}
