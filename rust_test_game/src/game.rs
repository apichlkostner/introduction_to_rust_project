use game_engine::*;

pub struct Pos {
    x: f32,
    y: f32,
}

pub struct Game {
    sprite: *mut ffi::Sprite,
    sprite_pos: Pos,
}

impl Game {
    pub fn new() -> Self {
        Self {
            sprite: spawn_sprite!(100.0, 100.0, 100, 100, 255, 0, 0),
            sprite_pos: Pos {
                x: -100.0,
                y: -100.0,
            },
        }
    }

    fn update_pos(&mut self) {
        if self.sprite_pos.x < 500.0 {
            self.sprite_pos.x += 1.0
        } else {
            self.sprite_pos.x = -100.0
        }
        if self.sprite_pos.y < 500.0 {
            self.sprite_pos.y += 1.0
        } else {
            self.sprite_pos.y = -100.0
        }
    }

    pub fn game_loop_start(&mut self) {
        self.update_pos();
        move_sprite!(self.sprite, self.sprite_pos.x, self.sprite_pos.y, true);
    }

    pub fn game_loop_end(&mut self) -> bool {
        true
    }
}
