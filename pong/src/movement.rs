use crate::sprite::Sprite;
use crate::world::World;

pub fn move_objects(world: &mut World, dt: f32) {
    for sprite in world.sprites.values_mut() {
        sprite.pos.x += dt * sprite.velocity.dx;
        sprite.pos.y += dt * sprite.velocity.dy;

        sprite.update_pos();
    }
}

pub fn collision(world: &mut World, _dt: f32) {
    let mut collisions = Vec::new();
    let mut names: Vec<String> = Vec::new();
    for name in world.sprites.keys().cloned() {
        names.push(name);
    }

    for (i, name_a) in names.iter().enumerate() {
        for name_b in &names[i + 1..] {
            let a = &world.sprites[name_a];
            let b = &world.sprites[name_b];

            if intersects(a, b) {
                collisions.push((name_a, name_b));
            }
        }
    }

    // resolve collisions
    for (name_a, name_b) in collisions {
        if let Some(sprite) = world.sprites.get_mut(name_a) {
            resolve_collision(sprite);
        }
        if let Some(sprite) = world.sprites.get_mut(name_b) {
            resolve_collision(sprite);
        }
    }

    for (_name, sprite) in &mut world.sprites {
        let mut collided = false;
        // Left or right border
        if sprite.pos.x < 0.0 || sprite.pos.x + sprite.size.width as f32 > world.window.width {
            sprite.velocity.dx = -sprite.velocity.dx;
            collided = true;
        }
        // Top or bottom border
        if sprite.pos.y < 0.0 || sprite.pos.y + sprite.size.height as f32 > world.window.height {
            sprite.velocity.dy = -sprite.velocity.dy;
            collided = true;
        }
        // Optionally, clamp position inside window if needed
        if collided {
            sprite.pos.x = sprite
                .pos
                .x
                .max(0.0)
                .min(world.window.width - sprite.size.width as f32);
            sprite.pos.y = sprite
                .pos
                .y
                .max(0.0)
                .min(world.window.height - sprite.size.height as f32);
        }
    }
}

fn resolve_collision(sprite: &mut Sprite) {
    sprite.velocity.dx = -sprite.velocity.dx;
    sprite.velocity.dy = -sprite.velocity.dy;
}

pub fn intersects(a: &Sprite, b: &Sprite) -> bool {
    a.pos.x < b.pos.x + b.size.width
        && a.pos.x + a.size.width > b.pos.x
        && a.pos.y < b.pos.y + b.size.height
        && a.pos.y + a.size.height > b.pos.y
}
