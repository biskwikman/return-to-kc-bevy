use crate::components::*;
use crate::rect::*;
use crate::resources::*;
use bevy::prelude::{
    default, AssetServer, Commands, Entity, JustifyText, ParamSet, Query, Res, ResMut, Text,
    Text2dBundle, TextStyle, Transform,
};
use rand::Rng;
use std::cmp::{max, min};

pub fn get_tile_idx(idx_xy: (usize, usize)) -> usize {
    idx_xy.0 + 80 * idx_xy.1
}

pub fn create_map(
    mut set: ParamSet<(Query<(Entity, &mut Tile, &Transform)>, Query<&mut Tile>)>,
    mut map: ResMut<Map>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tile_resolution: Res<TileResolution>,
    font_size: Res<FontSize>,
) {
    for (ent, mut tile, _transform) in set.p0().iter_mut() {
        tile.tiletype = TileType::Wall;
        map.tiles.push(ent);
    }

    let text_style = create_text_style(asset_server, font_size);

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = rand::thread_rng();

    let tile_width = tile_resolution.width;
    let tile_height = tile_resolution.height;
    for _ in 0..MAX_ROOMS {
        let w: usize = rng.gen_range(MIN_SIZE..=MAX_SIZE) as usize;
        let h: usize = rng.gen_range(MIN_SIZE..=MAX_SIZE) as usize;
        let x: usize = rng.gen_range(1..=tile_width - w - 1) - 1;
        let y: usize = rng.gen_range(1..=tile_height - h - 1) - 1;
        let new_room = Rect::new(x as i32, y as i32, (x + w) as i32, (y + h) as i32);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            apply_room_to_map(&new_room, &map, set.p1());

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.gen_range(0..2) == 1 {
                    apply_horizontal_tunnel(
                        &map,
                        prev_x,
                        new_x,
                        prev_y,
                        set.p1(),
                        &tile_resolution,
                    );
                    apply_vertical_tunnel(&map, prev_y, new_y, new_x, set.p1(), &tile_resolution);
                } else {
                    apply_horizontal_tunnel(&map, prev_x, new_x, new_y, set.p1(), &tile_resolution);
                    apply_vertical_tunnel(&map, prev_y, new_y, prev_x, set.p1(), &tile_resolution);
                }
            }
            rooms.push(new_room);
        }
    }

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

fn create_text_style(
    asset_server: Res<AssetServer>,
    // font: Handle<Font>,
    font_size: Res<FontSize>,
) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Mx437_IBM_BIOS.ttf"),
        font_size: font_size.0,
        ..default()
    }
}

fn apply_room_to_map(room: &Rect, map: &ResMut<Map>, mut query: Query<&mut Tile>) {
    for y in room.y0 as usize..=room.y1 as usize {
        for x in room.x0 as usize..=room.x1 as usize {
            query
                .get_mut(map.tiles[get_tile_idx((x, y))])
                .unwrap()
                .tiletype = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(
    map: &ResMut<Map>,
    x1: i32,
    x2: i32,
    y: i32,
    mut query: Query<&mut Tile>,
    tile_resolution: &Res<TileResolution>,
) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = get_tile_idx((x as usize, y as usize));
        if idx > 0 && idx < tile_resolution.width * tile_resolution.height {
            query.get_mut(map.tiles[idx]).unwrap().tiletype = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(
    map: &ResMut<Map>,
    y1: i32,
    y2: i32,
    x: i32,
    mut query: Query<&mut Tile>,
    tile_resolution: &Res<TileResolution>,
) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = get_tile_idx((x as usize, y as usize));
        if idx > 0 && idx < tile_resolution.width * tile_resolution.height {
            query.get_mut(map.tiles[idx]).unwrap().tiletype = TileType::Floor;
        }
    }
}
