use crate::world::World;
use game_engine::*;

/// Renders the player sprite and all other sprites in the world.
///
/// # Arguments
///
/// * `world` - A reference to the game world containing sprites to render.
pub fn render(world: &World) {
    for sprite_ref in world.get_sprites().values() {
        rust_render_sprite(sprite_ref.get_c_sprite());
    }
}
