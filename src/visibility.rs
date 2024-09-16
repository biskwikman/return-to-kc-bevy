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
    mut set: ParamSet<(Query<(&mut Tile, &Position)>, Query<&mut Tile>)>,
    mut query_player: Query<(&Position, &mut Viewshed), With<Player>>,
    map: Res<Map>,
) {
    let (player_pos, mut player_viewshed) = query_player.get_single_mut().unwrap();
    player_viewshed.visible_tiles.clear();
    let view_range = player_viewshed.range;

    // FOV direction crawl
    for (mut tile, _pos) in set.p0().iter_mut() {
        match tile.visibletype {
            VisibleType::Visible => {
                tile.visibletype = VisibleType::Memoried;
            }
            VisibleType::Memoried => {}

            _ => {}
        }
    }
    for depth in 1..=view_range {
        for breadth in (3..=(depth * 2) + 1).step_by(2) {
            for angle in breadth / -2..=breadth / 2 {
                let y = player_pos.y as i32 + depth;
                let x = player_pos.x as i32 + angle;
                if y > 0 && y < 60 && x > 0 && x < 80 {
                    if (set
                        .p0()
                        .get_mut(
                            map.tiles[get_tile_idx((
                                (player_pos.x as i32 + angle) as usize,
                                player_pos.y + depth as usize,
                            ))],
                        )
                        .unwrap())
                    .0
                    .tiletype
                        == TileType::Wall
                    {
                        let tile_y = set
                            .p0()
                            .get_mut(
                                map.tiles[get_tile_idx((
                                    (player_pos.x as i32 + angle) as usize,
                                    player_pos.y + depth as usize,
                                ))],
                            )
                            .unwrap()
                            .1
                            .y;
                        let tile_x = set
                            .p0()
                            .get_mut(
                                map.tiles[get_tile_idx((
                                    (player_pos.x as i32 + angle) as usize,
                                    player_pos.y + depth as usize,
                                ))],
                            )
                            .unwrap()
                            .1
                            .x;
                        let slope = (tile_y as f32 - player_pos.y as f32)
                            / (tile_x as f32 - player_pos.x as f32);
                        println!("{slope}");

                        // If wall is perpendicular

                        if slope == f32::INFINITY {
                            let mut nearest_wall = tile_y;
                            for i in tile_y..player_pos.y {
                                if set
                                    .p1()
                                    .get(map.tiles[get_tile_idx((tile_x, tile_y - i))])
                                    .unwrap()
                                    .tiletype
                                    == TileType::Wall
                                {
                                    nearest_wall = tile_y - i;
                                }
                            }
                            for y_diff in nearest_wall..=player_pos.y + view_range as usize {
                                if y_diff < 80 {
                                    set.p1()
                                        .get_mut(map.tiles[get_tile_idx((tile_x, y_diff))])
                                        .unwrap()
                                        .visibletype = VisibleType::Obscured;
                                }
                            }
                            let tile2tiletype = set
                                .p1()
                                .get(map.tiles[get_tile_idx((tile_x, tile_y - 1))])
                                .unwrap()
                                .tiletype;

                            if tile2tiletype != TileType::Wall {
                                set.p0()
                                    .get_mut(
                                        map.tiles[get_tile_idx((
                                            (player_pos.x as i32 + angle) as usize,
                                            player_pos.y + depth as usize,
                                        ))],
                                    )
                                    .unwrap()
                                    .0
                                    .visibletype = VisibleType::Visible;
                            }
                            // If wall is at 45 deg angle
                        } else if slope == 1. || slope == -1. {
                        }
                        continue;
                    } else {
                        set.p0()
                            .get_mut(
                                map.tiles[get_tile_idx((
                                    (player_pos.x as i32 + angle) as usize,
                                    player_pos.y + depth as usize,
                                ))],
                            )
                            .unwrap()
                            .0
                            .visibletype = VisibleType::Visible;
                    }
                }
            }
        }
    }
}

fn apply_view(
    mut query_viewshed: Query<&mut Viewshed>,
    mut query_text: Query<(&mut Text, &Tile)>,
    asset_server: Res<AssetServer>,
    font_size: Res<FontSize>,
) {
    let text_style_vis = create_text_style(
        &asset_server,
        &font_size,
        Srgba {
            red: 60.,
            green: 170.,
            blue: 70.,
            alpha: 1.,
        },
    );
    let text_style_mem = create_text_style(
        &asset_server,
        &font_size,
        Srgba {
            red: 255.,
            green: 255.,
            blue: 255.,
            alpha: 0.4,
        },
    );

    for (mut text, tile) in query_text.iter_mut() {
        match tile.visibletype {
            VisibleType::Visible => {
                if tile.tiletype == TileType::Floor {
                    text.sections = vec![TextSection::new('.', text_style_vis.clone()); 1];
                } else {
                    text.sections = vec![TextSection::new('#', text_style_vis.clone()); 1];
                }
            }
            VisibleType::Obscured => {
                text.sections = vec![TextSection::new(' ', text_style_vis.clone()); 1]
            }
            VisibleType::Memoried => {
                if tile.tiletype == TileType::Floor {
                    text.sections = vec![TextSection::new('.', text_style_mem.clone()); 1];
                } else {
                    text.sections = vec![TextSection::new('#', text_style_mem.clone()); 1];
                }
            }
            _ => {}
        }
    }
    // let viewshed = query_viewshed.get_single_mut().unwrap();
    // let mut iter = query_text.iter_many_mut(&viewshed.visible_tiles);
    // while let Some((mut text, tile)) = iter.fetch_next() {
    //     if tile.tiletype == TileType::Floor {
    //         text.sections = vec![TextSection::new('.', text_style.clone()); 1];
    //     } else {
    //         text.sections = vec![TextSection::new('#', text_style.clone()); 1];
    //     }
    // }
}
