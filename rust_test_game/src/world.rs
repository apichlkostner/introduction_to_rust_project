use crate::sprite::Sprite;
use game_engine::*;

pub struct World {
    player_sprite: Sprite,
    sprites: Vec<Sprite>,
}

impl World {
    pub fn empty() -> Self {
        Self{
            player_sprite: Sprite::new(0.0,0.0,0,0,0,0,0),
            sprites: Vec::new(),
        }
    }

    pub fn set_player_sprite(&mut self, x: f32, y: f32, width: i32, height: i32, r: i32, g: i32, b: i32) {
        self.player_sprite = Sprite::new(x,y,width,height,r,g,b);
    }

    pub fn add_sprite(&mut self, x: f32, y: f32, width: i32, height: i32, r: i32, g: i32, b: i32) {
        self.sprites.push(Sprite::new(x,y,width,height,r,g,b));
    }

    pub fn get_player_sprite(&self) -> &Sprite {
        &self.player_sprite
    }

    pub fn get_sprites(&self) -> &Vec<Sprite> {
        &self.sprites
    }

    pub fn move_player(&mut self, dx: f32, dy: f32)
    {
        self.player_sprite.x += dx;
        self.player_sprite.y += dy;
        move_sprite!(self.player_sprite.get_c_sprite(), self.player_sprite.x, self.player_sprite.y);
    }
}