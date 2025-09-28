use game_engine::*;

/// Represents an RGB color used to render sprites.
pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

/// Represents a position in 2D game world space.
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

/// Represents a velocity vector in the game world,
/// indicating how much the position changes per update step.
///
/// Currently unused, but kept for planned movement/physics logic.
#[allow(dead_code)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

/// Represents the size (width and height) of a sprite in pixels.
pub struct Size {
    pub width: i32,
    pub height: i32,
}

/// Represents a game sprite, which is a renderable object in the world..
pub struct Sprite {
    /// Pointer to the underlying C `Sprite` managed by the engine.
    c_sprite: *mut ffi::Sprite,

    /// Current position of the sprite in world space.
    pub pos: Pos,

    /// Movement velocity of the sprite.
    /// (Reserved for future updates; not currently used in game loop.)
    #[allow(dead_code)]
    pub speed: Velocity,

    /// Color of the sprite.
    /// (Reserved for future updates; not currently used in rendering logic.)
    #[allow(dead_code)]
    pub color: Color,
}

impl Sprite {
    /// Creates a new sprite and registers it with the game engine.
    ///
    /// This function spawns a sprite using the provided position, velocity,
    /// color, and size, and returns a managed `Sprite` wrapper around the
    /// engine’s underlying `ffi::Sprite` pointer.
    ///
    /// # Arguments
    ///
    /// * `pos` - The initial position of the sprite.
    /// * `speed` - The movement velocity of the sprite (currently unused).
    /// * `color` - The sprite’s color (currently unused, but passed to the engine).
    /// * `size` - The width and height of the sprite.
    pub fn new(pos: Pos, speed: Velocity, color: Color, size: Size) -> Self {
        // The `spawn_sprite!` macro allocates a C-side sprite
        // and returns a raw pointer. This pointer is stored in
        // `c_sprite` but is ultimately owned by the engine.
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

    /// Returns a raw pointer to the underlying C sprite.
    ///
    /// # Safety
    ///
    /// This pointer is owned and managed by the engine.
    /// Do not attempt to free, clone, or otherwise manage its memory directly.
    pub fn get_c_sprite(&self) -> *mut ffi::Sprite {
        self.c_sprite
    }

    /// Moves the sprite by the given delta values (`dx`, `dy`).
    ///
    /// Updates both the Rust-side position and the engine-side position
    /// by invoking the `move_sprite!` macro.
    ///
    /// # Arguments
    ///
    /// * `dx` - Change in the x-axis.
    /// * `dy` - Change in the y-axis.
    pub fn move_pos(&mut self, dx: f32, dy: f32) {
        // Update the Rust-side position first...
        self.pos.x += dx;
        self.pos.y += dy;

        // ...then update the engine-side representation
        // so the sprite is visually moved in the game world.
        move_sprite!(self.get_c_sprite(), self.pos.x, self.pos.y);
    }
}
