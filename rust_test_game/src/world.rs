use crate::sprite::Sprite;

pub struct World {
    pub player_sprite: Sprite,
    pub sprites: Vec<Sprite>,
}

impl World {
    pub fn empty() -> Self {
        Self{
            player_sprite: Sprite::new(0.0,0.0,0,0,0,0,0),
            sprites: Vec::new(),
        }
    }
    pub fn new(x: f32, y: f32, width: i32, height: i32, r: i32, g: i32, b: i32) -> Self {
        Self{
            player_sprite: Sprite::new(x,y,width,height,r,g,b),
            sprites: Vec::new(),
        }
    }
}