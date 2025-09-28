use crate::sprite::*;
use std::collections::HashMap;

/// Represents the game world, containing the player sprite and other sprites.
pub struct World {
    sprites: HashMap<String, Sprite>,
}

impl World {
    /// Creates an empty world with a default player sprite and no other sprites.
    pub fn empty() -> Self {
        Self {
            sprites: HashMap::new(),
        }
    }

    /// Adds a new sprite
    pub fn add_sprite(&mut self, name: &str, pos: Pos, size: Size, color: Color) {
        self.sprites.insert(
            String::from(name),
            Sprite::new(pos, Velocity { dx: 0.0, dy: 0.0 }, color, size),
        );
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sprite::{Color, Pos, Size};

    #[test]
    fn test_empty() {
        let world = World::empty();
        assert!(world.sprites.is_empty());
    }

    #[test]
    fn test_add_sprite() {
        let mut world = World::empty();
        let name = "player";
        let pos = Pos { x: 1.0, y: 2.0 };
        let size = Size {
            width: 10,
            height: 20,
        };
        let color = Color { r: 255, g: 128, b: 0 };

        world.add_sprite(name, pos, size, color);

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
        let name = "player";
        let pos = Pos { x: 1.0, y: 2.0 };
        let size = Size { width: 10, height: 20 };
        let color = Color { r: 255, g: 128, b: 0 };
        world.add_sprite(name, pos, size, color);

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
        world.add_sprite(
            "foo",
            Pos { x: 0.0, y: 0.0 },
            Size { width: 1, height: 1 },
            Color { r: 0, g: 0, b: 0 },
        );
        let sprites = world.get_sprites();
        assert_eq!(sprites.len(), 1);
        assert!(sprites.contains_key("foo"));
    }
}
