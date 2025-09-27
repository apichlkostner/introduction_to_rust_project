use crate::sprite::Sprite;

/// Represents the game world, containing the player sprite and other sprites.
pub struct World {
    player_sprite: Sprite,
    sprites: Vec<Sprite>,
}

impl World {
    /// Creates an empty world with a default player sprite and no other sprites.
    pub fn empty() -> Self {
        Self{
            player_sprite: Sprite::new(0.0,0.0,0,0,0,0,0),
            sprites: Vec::new(),
        }
    }

    /// Sets the player sprite's position, size, and color.
    ///
    /// # Arguments
    ///
    /// * `x` - The x position of the player sprite.
    /// * `y` - The y position of the player sprite.
    /// * `width` - The width of the player sprite.
    /// * `height` - The height of the player sprite.
    /// * `r` - The red color component.
    /// * `g` - The green color component.
    /// * `b` - The blue color component.
    pub fn set_player_sprite(&mut self, x: f32, y: f32, width: i32, height: i32, r: i32, g: i32, b: i32) {
        self.player_sprite = Sprite::new(x,y,width,height,r,g,b);
    }

    /// Adds a new sprite to the world with the given position, size, and color.
    ///
    /// # Arguments
    ///
    /// * `x` - The x position of the sprite.
    /// * `y` - The y position of the sprite.
    /// * `width` - The width of the sprite.
    /// * `height` - The height of the sprite.
    /// * `r` - The red color component.
    /// * `g` - The green color component.
    /// * `b` - The blue color component.
    pub fn add_sprite(&mut self, x: f32, y: f32, width: i32, height: i32, r: i32, g: i32, b: i32) {
        self.sprites.push(Sprite::new(x,y,width,height,r,g,b));
    }

    /// Returns a reference to the player sprite.
    pub fn get_player_sprite(&self) -> &Sprite {
        &self.player_sprite
    }

    /// Returns a reference to the vector of all sprites in the world.
    pub fn get_sprites(&self) -> &Vec<Sprite> {
        &self.sprites
    }

    /// Moves the player sprite by the given delta values
    ///
    /// # Arguments
    ///
    /// * `dx` - The change in x position.
    /// * `dy` - The change in y position.
    pub fn move_player(&mut self, dx: f32, dy: f32)
    {
        self.player_sprite.move_pos(dx, dy);
    }
}