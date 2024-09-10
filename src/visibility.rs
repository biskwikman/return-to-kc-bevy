use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

fn get_viewshed(
    query_tiles: Query<&Tile>,
    query_player: Query<&Position, With<Player>>,
    map: Res<Map>,
) {
    let player_pos = query_player.get_single().unwrap();
}
