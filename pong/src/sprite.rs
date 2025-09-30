//! Sprite definitions and logic for Pong.
//!
//! This module defines the core data structures for representing sprites (game objects),
//! including their position, size, velocity, and color. It also provides methods for
//! creating and manipulating sprites, and synchronizing their state with the game engine.

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
    pub width: f32,
    pub height: f32,
}

/// Represents a game sprite, which is a renderable object in the world.
pub struct Sprite {
    /// Pointer to the underlying C `Sprite` managed by the engine.
    c_sprite: *mut ffi::Sprite,

    /// Current position of the sprite in world space.
    pub pos: Pos,

    pub size: Size,

    /// Movement velocity of the sprite.
    /// (Reserved for future updates; not currently used in game loop.)
    #[allow(dead_code)]
    pub velocity: Velocity,

    /// Color of the sprite.
    /// (Reserved for future updates; not currently used in rendering logic.)
    #[allow(dead_code)]
    pub color: Color,
}

impl Sprite {
    /// Creates a new sprite and registers it with the game engine.
    ///
    /// Spawns a sprite using the provided position, velocity, color, and size,
    /// and returns a managed `Sprite` wrapper around the engine’s underlying `ffi::Sprite` pointer.
    ///
    /// # Arguments
    ///
    /// * `pos` - The initial position of the sprite.
    /// * `velocity` - The movement velocity of the sprite (currently unused).
    /// * `color` - The sprite’s color (currently unused, but passed to the engine).
    /// * `size` - The width and height of the sprite.
    pub fn new(pos: Pos, velocity: Velocity, color: Color, size: Size) -> Self {
        // The `spawn_sprite!` macro allocates a C-side sprite
        // and returns a raw pointer. This pointer is stored in
        // `c_sprite` but is ultimately owned by the engine.
        let sprite_ptr = spawn_sprite!(
            pos.x,
            pos.y,
            size.width as i32,
            size.height as i32,
            color.r,
            color.g,
            color.b
        );
        Self {
            c_sprite: sprite_ptr,
            pos,
            size,
            velocity,
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

 
    /// Updates the internal position of the sprite and synchronizes it with the engine-side
    pub fn set_pos(&mut self, pos: &Pos) {
        // Update the Rust-side position first...
    
        self.pos.x = pos.x;
        self.pos.y = pos.y;

        // ...then update the engine-side representation
        self.update_pos();
    }

    /// Synchronizes the engine-side sprite position with the Rust-side position.
    pub fn update_pos(&self) {
        move_sprite!(self.get_c_sprite(), self.pos.x, self.pos.y);
    }
}
