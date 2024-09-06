use crate::components::*;
use crate::rect::Intersect;
use crate::resources::*;
use bevy::prelude::*;
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

    for _ in 0..MAX_ROOMS {
        let w = rng.gen_range(MIN_SIZE..=MAX_SIZE);
        let h = rng.gen_range(MIN_SIZE..=MAX_SIZE);
        let x = rng.gen_range(1..=tile_resolution.width as i32 - w - 1) - 1;
        let y = rng.gen_range(1..=tile_resolution.height as i32 - h - 1) - 1;
        let new_room = Rect::new(x as f32, y as f32, (x + w) as f32, (y + h) as f32);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.is_intersect(other_room) {
                ok = false
            }
        }
        if ok {
            apply_room_to_map(, , );
            rooms.push(new_room);
        }
    }

    let room1 = Rect::new(0., 0., 35., 25.);
    let room2 = Rect::new(50., 10., 59., 50.);

    apply_room_to_map(&room1, &map, set.p1());
    apply_room_to_map(&room2, &map, set.p1());
    apply_horizontal_tunnel(&map, 35, 50, 20, set.p1(), tile_resolution);

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
    for y in room.min.y as usize..=room.max.y as usize {
        for x in room.min.x as usize..=room.max.x as usize {
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
    tile_resolution: Res<TileResolution>,
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
    tile_resolution: Res<TileResolution>,
) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = get_tile_idx((x as usize, y as usize));
        if idx > 0 && idx < tile_resolution.width * tile_resolution.height {
            query.get_mut(map.tiles[idx]).unwrap().tiletype = TileType::Floor;
        }
    }
}
