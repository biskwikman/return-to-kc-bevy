use crate::components::*;
use crate::get_tile_idx;
use crate::resources::*;
use bevy::prelude::*;

fn get_viewshed(
    mut query_tiles: Query<Entity, With<Tile>>,
    mut query_player: Query<(&Position, &mut Viewshed), With<Player>>,
    map: Res<Map>,
) {
    let (player_pos, mut player_viewshed) = query_player.get_single_mut().unwrap();
    let view_range = player_viewshed.range;

    let mut i_x = 1;
    for y_range in (view_range * -1)..=view_range {
        let y = player_pos.y + y_range as usize;
        for _x_range in (view_range * -1)..=view_range {
            for x in 1..=i_x {
                let vis_tile_ent = query_tiles
                    .get_mut(map.tiles[get_tile_idx((x, y))])
                    .unwrap();
                player_viewshed.visible_tiles.push(vis_tile_ent);
            }
        }

        // let test = map.tiles[get_tile_idx((player_pos.x + x as usize, player_pos.y))];
        i_x = i_x + 1;
    }
}
