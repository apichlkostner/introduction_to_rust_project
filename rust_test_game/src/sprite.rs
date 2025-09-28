use game_engine::*;

/// Represents a sprite in the game world, holding its position and a pointer to the underlying C sprite.
pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

pub struct Pos {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

pub struct Size {
    pub width: i32,
    pub height: i32,
}

/// Represents a sprite in the game world, holding its position and a pointer to the underlying C sprite.
pub struct Sprite {
    c_sprite: *mut ffi::Sprite,
    pub pos: Pos,
    #[allow(dead_code)]
    pub speed: Velocity, // for later code updates
    #[allow(dead_code)]
    pub color: Color, // for later code updates
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
    pub fn new(pos: Pos, speed: Velocity, color: Color, size: Size) -> Self {
        let sprite_ptr = spawn_sprite!(
            pos.x,
            pos.y,
            size.width,
            size.height,
            color.r,
            color.g,
            color.b
        );
        Self {
            c_sprite: sprite_ptr,
            pos,
            speed,
            color,
        }
    }

    /// Returns a pointer to the underlying C sprite.
    pub fn get_c_sprite(&self) -> *mut ffi::Sprite {
        return self.c_sprite;
    }

    pub fn move_pos(&mut self, dx: f32, dy: f32) {
        self.pos.x += dx;
        self.pos.y += dy;
        move_sprite!(self.get_c_sprite(), self.pos.x, self.pos.y);
    }
}
