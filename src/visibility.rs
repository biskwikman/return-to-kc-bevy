use crate::components::*;
use crate::create_text_style;
use crate::get_tile_idx;
use crate::resources::*;
use bevy::prelude::*;

pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (get_viewshed, apply_view.after(get_viewshed)));
    }
}

fn get_viewshed(
    mut query_tiles: Query<(Entity, &Tile, &Position)>,
    mut query_player: Query<(&Position, &mut Viewshed), With<Player>>,
    map: Res<Map>,
) {
    let (player_pos, mut player_viewshed) = query_player.get_single_mut().unwrap();
    player_viewshed.visible_tiles.clear();
    let view_range = player_viewshed.range;

    // FOV direction crawl
    let mut obscured_tiles: Vec<(i32, i32)> = Vec::new();
    for depth in 1..=view_range {
        for breadth in (3..=(depth * 2) + 1).step_by(2) {
            for angle in breadth / -2..=breadth / 2 {
                let y = player_pos.y as i32 + depth;
                let x = player_pos.x as i32 + angle;
                if y > 0 && y < 60 && x > 0 && x < 80 && !obscured_tiles.contains(&(x, y)) {
                    let tile = query_tiles
                        .get_mut(
                            map.tiles[get_tile_idx((
                                (player_pos.x as i32 + angle) as usize,
                                player_pos.y + depth as usize,
                            ))],
                        )
                        .unwrap();

                    if tile.1.tiletype == TileType::Wall {
                        player_viewshed.visible_tiles.push(tile.0);
                        let slope = (tile.2.y as f32 - player_pos.y as f32)
                            / (tile.2.x as f32 - player_pos.x as f32);
                        println!("{slope}");

                        // If wall is perpendicular
                        if slope == f32::INFINITY {
                            for sub_depth in 1..=view_range - depth {
                                for sub_breadth in (3..=(sub_depth * 2) + 1).step_by(2) {
                                    for sub_angle in sub_breadth / -2..=sub_breadth / 2 {
                                        let sub_y = tile.2.y as i32 + sub_depth;
                                        let sub_x = tile.2.x as i32 + sub_angle;
                                        obscured_tiles.push((sub_x, sub_y));
                                    }
                                }
                            }
                        } else if slope == 1. {
                        }
                        continue;
                    } else {
                        player_viewshed.visible_tiles.push(tile.0);
                    }
                }
            }
        }
    }

    // Infill
    // for iy in -down_limit..=up_limit {
    //     for ix in -left_limit..=right_limit {
    //         let vis_tile = query_tiles
    //             .get_mut(
    //                 map.tiles[get_tile_idx((
    //                     (player_pos.x as i32 + ix) as usize,
    //                     (player_pos.y as i32 + iy) as usize,
    //                 ))],
    //             )
    //             .unwrap();
    //         player_viewshed.visible_tiles.push(vis_tile.0);
    //     }
    // }

    // let mut i_x: i32 = 1;
    // let mut halfway = false;
    // for y_range in -view_range..=view_range {
    //     // if y_range > up_limit || y_range < -down_limit {
    //     //     continue;
    //     // }
    //     let y = player_pos.y as i32 + y_range;

    //     for (i, _x_i2) in (1..=i_x).enumerate() {
    //         let x_offsets: Vec<i32> = ((i_x / 2) - (i_x - 1)..=(i_x / 2)).collect();
    //         let x = player_pos.x as i32 + x_offsets[i];
    //         if x < -left_limit || x > right_limit {}
    //         if x >= 0 && x < 80 && y >= 0 && y < 60 {
    //             let vis_tile = query_tiles
    //                 .get_mut(map.tiles[get_tile_idx((x as usize, y as usize))])
    //                 .unwrap();
    //             player_viewshed.visible_tiles.push(vis_tile.0);
    //         }
    //     }
    //     if i_x < ((view_range * 2) + 1) && halfway == false {
    //         i_x += 2;
    //     } else if i_x == ((view_range * 2) + 1) && halfway == false {
    //         i_x -= 2;
    //         halfway = true;
    //     } else if halfway == true {
    //         i_x -= 2;
    //     }
    // }
}

fn apply_view(
    mut query_viewshed: Query<&mut Viewshed>,
    mut query_text: Query<(&mut Text, &Tile)>,
    asset_server: Res<AssetServer>,
    font_size: Res<FontSize>,
) {
    let text_style = create_text_style(asset_server, font_size);

    let viewshed = query_viewshed.get_single_mut().unwrap();
    let mut iter = query_text.iter_many_mut(&viewshed.visible_tiles);
    while let Some((mut text, tile)) = iter.fetch_next() {
        if tile.tiletype == TileType::Floor {
            text.sections = vec![TextSection::new('.', text_style.clone()); 1];
        } else {
            text.sections = vec![TextSection::new('#', text_style.clone()); 1];
        }
    }
}
