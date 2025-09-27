use crate::world::World;
use game_engine::*;

/// Renders the player sprite and all other sprites in the world.
///
/// # Arguments
///
/// * `world` - A reference to the game world containing sprites to render.
pub fn render(world: &World) {
    rust_render_sprite(world.get_player_sprite().get_c_sprite());
    for sprite_ref in world.get_sprites() {
        rust_render_sprite(sprite_ref.get_c_sprite());
    }
}
