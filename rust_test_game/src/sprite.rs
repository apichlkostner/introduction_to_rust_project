use game_engine::*;

pub struct Sprite {
    c_sprite: *mut ffi::Sprite,
    pub x: f32,
    pub y: f32,
}

impl Sprite {
    pub fn new(x: f32, y: f32, width: i32, height: i32, r: i32, g: i32, b: i32) -> Self {
        let sprite_ptr = spawn_sprite!(x, y, width, height, r, g, b);
        Self {
            c_sprite: sprite_ptr,
            x: x,
            y: y,
        }
    }

    pub fn get_c_sprite(&self) -> *mut ffi::Sprite{
        return self.c_sprite;
    }
}
