//! Game world logic for Pong.
//!
//! This module defines the `World` struct, which manages all sprites and the game window.
//! It provides methods for adding, moving, and retrieving sprites.

use crate::sprite::*;
use std::collections::HashMap;

/// Represents the game world, containing all sprites and the window size.
pub struct World {
    pub sprites: HashMap<String, Sprite>,
    pub window: Size,
}

impl World {
    /// Creates an empty world with no sprites and a default window size.
    ///
    /// # Returns
    ///
    /// A new `World` instance.
    pub fn empty() -> Self {
        Self {
            sprites: HashMap::new(),
            window: Size {
                width: 1024.0,
                height: 768.0,
            },
        }
    }

    /// Adds a new sprite to the world.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sprite.
    /// * `pos` - The initial position of the sprite.
    /// * `velocity` - The velocity of the sprite.
    /// * `size` - The size of the sprite.
    /// * `color` - The color of the sprite.
    pub fn add_sprite(
        &mut self,
        name: &str,
        pos: Pos,
        velocity: Velocity,
        size: Size,
        color: Color,
    ) {
        self.sprites
            .insert(String::from(name), Sprite::new(pos, velocity, color, size));
    }

    /// Moves the sprite with the given name by the specified delta position.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sprite to move.
    /// * `delta_pos` - The change in position to apply.
    pub fn move_sprite(&mut self, name: &str, delta_pos: Pos) {
        if let Some(sprite) = self.sprites.get_mut(name) {
            sprite.move_pos(delta_pos.x, delta_pos.y);
        } else {
            // sprite not available
        }
    }

    /// Returns a reference to the hashmap of all sprites in the world.
    pub fn get_sprites(&self) -> &HashMap<String, Sprite> {
        &self.sprites
    }

    /// Returns a reference to the sprite with the given name.
    ///
    /// # Panics
    ///
    /// Panics if the sprite does not exist.
    pub fn get_sprite(&self, name: &str) -> &Sprite {
        &self.sprites[name]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sprite::{Color, Pos, Size, Velocity};

    #[test]
    fn test_empty() {
        let world = World::empty();
        assert!(world.sprites.is_empty());
    }

    #[test]
    fn test_add_sprite() {
        let mut world = World::empty();
        let name = "player1";
        let pos = Pos { x: 1.0, y: 2.0 };
        let size = Size {
            width: 10.0,
            height: 20.0,
        };
        let color = Color {
            r: 255,
            g: 128,
            b: 0,
        };
        let velocity = Velocity { dx: 0.0, dy: 0.0 };

        world.add_sprite(name, pos, velocity, size, color);

        // Check if the sprite was added
        assert!(world.sprites.contains_key(name));
        let sprite = world.sprites.get(name).unwrap();
        assert_eq!(sprite.pos.x, 1.0);
        assert_eq!(sprite.pos.y, 2.0);
        assert_eq!(sprite.color.r, 255);
        assert_eq!(sprite.color.g, 128);
        assert_eq!(sprite.color.b, 0);
    }

    #[test]
    fn test_move_sprite() {
        let mut world = World::empty();
        let name = "player1";
        let pos = Pos { x: 1.0, y: 2.0 };
        let size = Size {
            width: 10.0,
            height: 20.0,
        };
        let color = Color {
            r: 255,
            g: 128,
            b: 0,
        };
        let velocity = Velocity { dx: 0.0, dy: 0.0 };
        world.add_sprite(name, pos, velocity, size, color);

        let delta = Pos { x: 3.0, y: -1.0 };
        world.move_sprite(name, delta);

        let sprite = world.sprites.get(name).unwrap();
        assert_eq!(sprite.pos.x, 4.0);
        assert_eq!(sprite.pos.y, 1.0);
    }

    #[test]
    fn test_move_sprite_nonexistent() {
        let mut world = World::empty();
        // Should not panic or add anything
        world.move_sprite("ghost", Pos { x: 1.0, y: 1.0 });
        assert!(world.sprites.is_empty());
    }

    #[test]
    fn test_get_sprites() {
        let mut world = World::empty();
        let velocity = Velocity { dx: 0.0, dy: 0.0 };
        world.add_sprite(
            "foo",
            Pos { x: 0.0, y: 0.0 },
            velocity,
            Size {
                width: 1.0,
                height: 1.0,
            },
            Color { r: 0, g: 0, b: 0 },
        );
        let sprites = world.get_sprites();
        assert_eq!(sprites.len(), 1);
        assert!(sprites.contains_key("foo"));
    }
}
