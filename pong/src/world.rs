//! Game world logic for Pong.
//!
//! This module defines the [`World`] struct, which manages all sprites in the game
//! as well as the window size. It provides methods for creating, adding, moving,
//! and retrieving sprites by name.
//!
//! A `World` stores its sprites in a `HashMap<String, Sprite>`, where each sprite
//! is identified by a unique string key.

use crate::sprite::*;
use std::collections::HashMap;

/// Represents the game world, containing all sprites and the window size.
pub struct World {
    /// A collection of sprites in the world, keyed by name.
    pub sprites: HashMap<String, Sprite>,
    /// The dimensions of the game window.
    pub window: Size,
}

impl World {
    /// Creates an empty world with no sprites and a default window size of
    /// `1024x768` pixels.
    ///
    /// # Returns
    ///
    /// A new [`World`] instance with no sprites.
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
    /// If a sprite with the same name already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `name` - The unique identifier for the sprite.
    /// * `pos` - The initial position of the sprite.
    /// * `velocity` - The velocity of the sprite.
    /// * `size` - The dimensions of the sprite.
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

    /// Updates the position of a sprite in the world.
    ///
    /// If the sprite with the given name does not exist, this method does nothing.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sprite to update.
    /// * `pos` - The new position of the sprite.
    pub fn set_sprite_pos(&mut self, name: &str, pos: Pos) {
        if let Some(sprite) = self.sprites.get_mut(name) {
            sprite.set_pos(&pos);
        }
    }


    /// Returns a reference to all sprites in the world.
    ///
    /// # Returns
    ///
    /// A reference to the underlying [`HashMap`] of sprites.
    pub fn get_sprites(&self) -> &HashMap<String, Sprite> {
        &self.sprites
    }

    /// Returns a reference to a sprite by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sprite to retrieve.
    ///
    /// # Panics
    ///
    /// Panics if the sprite does not exist in the world.
    pub fn get_sprite(&self, name: &str) -> &Sprite {
        &self.sprites[name]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sprite::{Color, Pos, Size, Velocity};

    /// Verify that a new world starts with no sprites.
    #[test]
    fn test_empty() {
        let world = World::empty();
        assert!(world.sprites.is_empty());
    }

    /// Verify that a sprite can be added and its properties are stored correctly.
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

        assert!(world.sprites.contains_key(name));
        let sprite = world.sprites.get(name).unwrap();
        assert_eq!(sprite.pos.x, 1.0);
        assert_eq!(sprite.pos.y, 2.0);
        assert_eq!(sprite.color.r, 255);
        assert_eq!(sprite.color.g, 128);
        assert_eq!(sprite.color.b, 0);
    }


    /// Verify that `get_sprites` returns the correct collection of sprites.
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

        /// Verify that `set_sprite_pos` updates the sprite's position.
    #[test]
    fn test_set_sprite_pos() {
        let mut world = World::empty();
        world.add_sprite(
            "ball",
            Pos { x: 5.0, y: 5.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            Size { width: 2.0, height: 2.0 },
            Color { r: 255, g: 255, b: 255 },
        );

        world.set_sprite_pos("ball", Pos { x: 10.0, y: 15.0 });

        let sprite = world.get_sprite("ball");
        assert_eq!(sprite.pos.x, 10.0);
        assert_eq!(sprite.pos.y, 15.0);
    }

    /// Verify that `set_sprite_pos` does nothing if the sprite does not exist.
    #[test]
    fn test_set_sprite_pos_nonexistent() {
        let mut world = World::empty();
        world.set_sprite_pos("ghost", Pos { x: 10.0, y: 10.0 });
        // Still empty
        assert!(world.sprites.is_empty());
    }

    /// Verify that `get_sprite` retrieves the correct sprite.
    #[test]
    fn test_get_sprite() {
        let mut world = World::empty();
        world.add_sprite(
            "paddle",
            Pos { x: 3.0, y: 4.0 },
            Velocity { dx: 1.0, dy: 1.0 },
            Size { width: 5.0, height: 10.0 },
            Color { r: 0, g: 0, b: 255 },
        );

        let sprite = world.get_sprite("paddle");
        assert_eq!(sprite.pos.x, 3.0);
        assert_eq!(sprite.size.width, 5.0);
        assert_eq!(sprite.color.b, 255);
    }

    /// Verify that `get_sprite` panics if the sprite does not exist.
    #[test]
    #[should_panic]
    fn test_get_sprite_nonexistent_panics() {
        let world = World::empty();
        world.get_sprite("not_there");
    }

    /// Verify that adding a sprite with the same name replaces the old one.
    #[test]
    fn test_add_sprite_overwrites() {
        let mut world = World::empty();
        world.add_sprite(
            "ball",
            Pos { x: 1.0, y: 1.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            Size { width: 1.0, height: 1.0 },
            Color { r: 255, g: 0, b: 0 },
        );
        world.add_sprite(
            "ball",
            Pos { x: 2.0, y: 2.0 },
            Velocity { dx: 1.0, dy: 1.0 },
            Size { width: 2.0, height: 2.0 },
            Color { r: 0, g: 255, b: 0 },
        );

        let sprite = world.get_sprite("ball");
        assert_eq!(sprite.pos.x, 2.0);
        assert_eq!(sprite.color.g, 255);
        assert_eq!(sprite.size.width, 2.0);
    }

    /// Verify that the default window size is 1024x768.
    #[test]
    fn test_default_window_size() {
        let world = World::empty();
        assert_eq!(world.window.width, 1024.0);
        assert_eq!(world.window.height, 768.0);
    }
}
