use game_engine::*;

/// Represents a sprite in the game world, holding its position and a pointer to the underlying C sprite.
pub struct Sprite {
    c_sprite: *mut ffi::Sprite,
    /// The x position of the sprite.
    pub x: f32,
    /// The y position of the sprite.
    pub y: f32,
}

impl Sprite {
    /// Creates a new sprite with the given position, size, and color.
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
    pub fn new(x: f32, y: f32, width: i32, height: i32, r: i32, g: i32, b: i32) -> Self {
        let sprite_ptr = spawn_sprite!(x, y, width, height, r, g, b);
        Self {
            c_sprite: sprite_ptr,
            x: x,
            y: y,
        }
    }

    /// Returns a pointer to the underlying C sprite.
    pub fn get_c_sprite(&self) -> *mut ffi::Sprite{
        return self.c_sprite;
    }

    pub fn move_pos(&mut self, dx: f32, dy: f32){
        self.x += dx;
        self.y += dy;
        move_sprite!(self.get_c_sprite(), self.x, self.y);
    }
}
