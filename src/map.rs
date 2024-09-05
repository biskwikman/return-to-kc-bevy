use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::cmp::{max, min};

pub fn get_tile_idx(idx_xy: (usize, usize)) -> usize {
    idx_xy.0 + 80 * idx_xy.1
}

pub fn create_map(
    mut set: ParamSet<(Query<(Entity, &mut Tile, &Transform)>, Query<&mut Tile>)>,
    mut map: ResMut<Map>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Mx437_IBM_BIOS.ttf");
    let font_size = 10.0;
    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        ..default()
    };
    for (ent, mut tile, _transform) in set.p0().iter_mut() {
        tile.tiletype = TileType::Wall;
        map.tiles.push(ent);
    }

    let room1 = Rect::new(0., 0., 35., 25.);
    let room2 = Rect::new(50., 10., 59., 50.);

    apply_room_to_map(&room1, &map, set.p1());
    apply_room_to_map(&room2, &map, set.p1());
    apply_horizontal_tunnel(&map, 35, 50, 20, set.p1());

    for (ent, tile, transform) in set.p0().iter() {
        match tile.tiletype {
            TileType::Wall => {
                commands.entity(ent).insert(Text2dBundle {
                    text: Text::from_section('#', text_style.clone())
                        .with_justify(JustifyText::Center),
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y,
                        1.0,
                    ),
                    ..default()
                });
            }
            TileType::Floor => {}
        }
    }
}

pub fn apply_room_to_map(room: &Rect, map: &ResMut<Map>, mut query: Query<&mut Tile>) {
    for y in room.min.y as usize..=room.max.y as usize {
        for x in room.min.x as usize..=room.max.x as usize {
            query
                .get_mut(map.tiles[get_tile_idx((x, y))])
                .unwrap()
                .tiletype = TileType::Floor;
        }
    }
}

pub fn apply_horizontal_tunnel(
    map: &ResMut<Map>,
    x1: i32,
    x2: i32,
    y: i32,
    mut query: Query<&mut Tile>,
) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = get_tile_idx((x as usize, y as usize));
        if idx > 0 && idx < 80 * 50 {
            query.get_mut(map.tiles[idx]).unwrap().tiletype = TileType::Floor;
        }
    }
}

pub fn apply_vertical_tunnel(
    map: &ResMut<Map>,
    y1: i32,
    y2: i32,
    x: i32,
    mut query: Query<&mut Tile>,
) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = get_tile_idx((x as usize, y as usize));
        if idx > 0 && idx < 80 * 50 {
            query.get_mut(map.tiles[idx]).unwrap().tiletype = TileType::Floor;
        }
    }
}
