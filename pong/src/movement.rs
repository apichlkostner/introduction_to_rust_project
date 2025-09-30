//! Movement and collision logic for Pong.
//!
//! This module provides functions to move sprites, detect and resolve collisions
//! between sprites and with the game window borders.

use crate::sprite::Sprite;
use crate::world::World;

pub enum CollisionType {
    WithSprite,
    WithBorder,
}

/// Moves all sprites in the world according to their velocities and the time delta.
///
/// # Arguments
///
/// * `world` - Mutable reference to the game world containing all sprites.
/// * `dt` - The time delta since the last update (in milliseconds).
pub fn move_objects(world: &mut World, dt: f32) {
    for sprite in world.sprites.values_mut() {
        sprite.pos.x += dt * sprite.velocity.dx;
        sprite.pos.y += dt * sprite.velocity.dy;

        sprite.update_pos();
    }
}

/// Detects and resolves collisions between sprites and with the window borders.
///
/// For each pair of sprites, checks for intersection and inverts their velocities if they collide.
/// Also checks for collisions with the window borders and inverts the corresponding velocity component.
/// Returns `true` if any collision occurred.
///
/// # Arguments
///
/// * `world` - Mutable reference to the game world containing all sprites.
/// * `_dt` - The time delta since the last update (unused).
///
/// # Returns
///
/// * `true` if any collision occurred, otherwise `false`.
pub fn collision(world: &mut World, _dt: f32) -> Option<CollisionType> {
    let mut collisions = Vec::new();
    let mut names: Vec<String> = Vec::new();
    for name in world.sprites.keys().cloned() {
        names.push(name);
    }

    let mut collision_happened = None;

    for (i, name_a) in names.iter().enumerate() {
        for name_b in &names[i + 1..] {
            let a = &world.sprites[name_a];
            let b = &world.sprites[name_b];

            if intersects(a, b) {
                collision_happened = Some(CollisionType::WithSprite);
                collisions.push((name_a, name_b));
            }
        }
    }

    // resolve collisions
    for (name_a, name_b) in collisions {
        if let Some(sprite) = world.sprites.get_mut(name_a) {
            resolve_collision(sprite);
        }
        if let Some(sprite) = world.sprites.get_mut(name_b) {
            resolve_collision(sprite);
        }
    }

    // collision with window borders
    for (_name, sprite) in &mut world.sprites {
        let mut collided = false;
        
        // Left or right border
        if sprite.pos.x < 0.0 || sprite.pos.x + sprite.size.width as f32 > world.window.width {
            sprite.velocity.dx = -sprite.velocity.dx;
            collided = true;
        }
        
        // Top or bottom border
        if sprite.pos.y < 0.0 || sprite.pos.y + sprite.size.height as f32 > world.window.height {
            sprite.velocity.dy = -sprite.velocity.dy;
            collided = true;
        }

        if collided {
            collision_happened = Some(CollisionType::WithBorder);
            sprite.pos.x = sprite
                .pos
                .x
                .max(0.0)
                .min(world.window.width - sprite.size.width as f32);
            sprite.pos.y = sprite
                .pos
                .y
                .max(0.0)
                .min(world.window.height - sprite.size.height as f32);
        }
    }

    collision_happened
}

/// Inverts both velocity components of the given sprite to resolve a collision.
///
/// # Arguments
///
/// * `sprite` - Mutable reference to the sprite to update.
fn resolve_collision(sprite: &mut Sprite) {
    sprite.velocity.dx = -sprite.velocity.dx;
    sprite.velocity.dy = -sprite.velocity.dy;
}

/// Checks if two sprites intersect (axis-aligned bounding box collision).
///
/// # Arguments
///
/// * `a` - Reference to the first sprite.
/// * `b` - Reference to the second sprite.
///
/// # Returns
///
/// * `true` if the sprites intersect, otherwise `false`.
pub fn intersects(a: &Sprite, b: &Sprite) -> bool {
    a.pos.x < b.pos.x + b.size.width
        && a.pos.x + a.size.width > b.pos.x
        && a.pos.y < b.pos.y + b.size.height
        && a.pos.y + a.size.height > b.pos.y
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sprite::{Color, Pos, Size, Velocity};
    use crate::world::World;

    fn make_sprite(pos: Pos, velocity: Velocity, size: Size) -> crate::sprite::Sprite {
        let color = Color { r: 0, g: 0, b: 0 };
        crate::sprite::Sprite::new(pos, velocity, color, size)
    }

    #[test]
    fn test_move_objects_moves_sprite() {
        let mut world = World::empty();
        let velocity = Velocity { dx: 1.0, dy: 2.0 };
        let pos = Pos { x: 0.0, y: 0.0 };
        let size = Size { width: 10.0, height: 10.0 };
        world.sprites.insert(
            "test".to_string(),
            make_sprite(pos, velocity, size),
        );
        move_objects(&mut world, 1.0);
        let sprite = &world.sprites["test"];
        assert_eq!(sprite.pos.x, 1.0);
        assert_eq!(sprite.pos.y, 2.0);
    }

    #[test]
    fn test_intersects_true() {
        let a = make_sprite(
            Pos { x: 0.0, y: 0.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            Size { width: 10.0, height: 10.0 },
        );
        let b = make_sprite(
            Pos { x: 5.0, y: 5.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            Size { width: 10.0, height: 10.0 },
        );
        assert!(intersects(&a, &b));
    }

    #[test]
    fn test_intersects_false() {
        let a = make_sprite(
            Pos { x: 0.0, y: 0.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            Size { width: 10.0, height: 10.0 },
        );
        let b = make_sprite(
            Pos { x: 20.0, y: 20.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            Size { width: 5.0, height: 5.0 },
        );
        assert!(!intersects(&a, &b));
    }

    #[test]
    fn test_collision_between_sprites() {
        let mut world = World::empty();
        let velocity = Velocity { dx: 0.0, dy: 0.0 };
        let velocity2 = Velocity { dx: 0.0, dy: 0.0 };
        let size = Size { width: 10.0, height: 10.0 };
        let size2 = Size { width: 10.0, height: 10.0 };
        world.sprites.insert(
            "a".to_string(),
            make_sprite(Pos { x: 0.0, y: 0.0 }, velocity, size),
        );
        world.sprites.insert(
            "b".to_string(),
            make_sprite(Pos { x: 5.0, y: 5.0 }, velocity2, size2),
        );
        let collided = collision(&mut world, 0.0);
        assert!(matches!(collided, Some(CollisionType::WithSprite)));
    }

    #[test]
    fn test_collision_with_border() {
        let mut world = World::empty();
        let velocity = Velocity { dx: 5.0, dy: 0.0 };
        let size = Size { width: 10.0, height: 10.0 };
        // Place sprite near right border
        let pos = Pos { x: world.window.width + 9.0, y: 100.0 };
        world.sprites.insert(
            "border".to_string(),
            make_sprite(pos, velocity, size),
        );
        let collided = collision(&mut world, 0.0);
        // currently collision with border is not returned
        assert!(matches!(collided, Some(CollisionType::WithBorder)));
        let sprite = &world.sprites["border"];
        // Should be clamped within window
        assert!(sprite.pos.x <= world.window.width - sprite.size.width);
        // Velocity should be inverted
        assert_eq!(sprite.velocity.dx, -5.0);
    }
}
