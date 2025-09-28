use crate::sprite::*;

/// Represents the game world, containing the player sprite and other sprites.
pub struct World {
    player_sprite: Sprite,
    sprites: Vec<Sprite>,
}

impl World {
    /// Creates an empty world with a default player sprite and no other sprites.
    pub fn empty() -> Self {
        Self {
            player_sprite: Sprite::new(
                Pos { x: 0.0, y: 0.0 },
                Velocity { dx: 0.0, dy: 0.0 },
                Color { r: 0, g: 0, b: 0 },
                Size { width: 0, height: 0 },
            ),
            sprites: Vec::new(),
        }
    }

    /// Sets the player sprite's position, size, and color.
    pub fn set_player_sprite(&mut self, pos: Pos, size: Size, color: Color) {
        self.player_sprite = Sprite::new(
            pos,
            Velocity { dx: 0.0, dy: 0.0 },
            color,
            size,
        );
    }

    /// Adds a new sprite to the world with the given position, size, and color.
    pub fn add_sprite(&mut self, pos: Pos, size: Size, color: Color) {
        self.sprites.push(Sprite::new(
            pos,
            Velocity { dx: 0.0, dy: 0.0 },
            color,
            size,
        ));
    }

    /// Returns a reference to the player sprite.
    pub fn get_player_sprite(&self) -> &Sprite {
        &self.player_sprite
    }

    /// Returns a reference to the vector of all sprites in the world.
    pub fn get_sprites(&self) -> &Vec<Sprite> {
        &self.sprites
    }

    /// Moves the player sprite by the given delta values.
    pub fn move_player(&mut self, dx: f32, dy: f32) {
        self.player_sprite.move_pos(Pos{x: dx, y:dy});
    }
}
