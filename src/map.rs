use crate::components::*;
use crate::rect::*;
use crate::resources::*;
use bevy::color::Srgba;
use bevy::prelude::{
    default, App, AssetServer, Commands, Entity, IntoSystemConfigs, JustifyText, ParamSet, Plugin,
    Query, Res, ResMut, Startup, Text, Text2dBundle, TextStyle, Transform, Vec3, Window,
};
use rand::Rng;
use std::cmp::{max, min};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                apply_map,
                populate_map_resources.before(apply_map),
                create_map.before(populate_map_resources),
            ),
        );
    }
}

pub fn get_tile_idx(idx_xy: (usize, usize)) -> usize {
    idx_xy.0 + 80 * idx_xy.1
}

fn populate_map_resources(
    mut query_tiles: Query<(Entity, &mut Tile)>,
    query_rooms: Query<(Entity, &Room)>,
    mut map: ResMut<Map>,
) {
    for (ent, mut tile) in query_tiles.iter_mut() {
        tile.tiletype = TileType::Wall;
        map.tiles.push(ent);
    }

    for (ent, _room) in query_rooms.iter() {
        map.rooms.push(ent);
    }
}

fn create_map(
    mut commands: Commands,
    query_window: Query<&Window>,
    font_size: Res<FontSize>,
    mut tile_resolution: ResMut<TileResolution>,
) {
    // Create tiles
    let window = query_window.single();
    tile_resolution.width = (window.resolution.width() / font_size.0) as usize;
    tile_resolution.height = (window.resolution.height() / font_size.0) as usize;
    let y_max = window.resolution.height() / 2.0;
    let y_min = window.resolution.height() / -2.0 + font_size.0 / 2.0;
    let x_max = window.resolution.width() / 2.0;
    let x_min = window.resolution.width() / -2.0 + font_size.0 / 2.0;
    let x_range = (x_min as i32..x_max as i32).step_by(font_size.0 as usize);

    for (iy, y) in (y_min as i32..y_max as i32)
        .step_by(font_size.0 as usize)
        .enumerate()
    {
        for (ix, x) in x_range.clone().enumerate() {
            commands.spawn((
                Tile {
                    zone: PlayerZone::Outside,
                    tiletype: TileType::Floor,
                    visibletype: VisibleType::Undiscovered,
                },
                Position { x: ix, y: iy },
                Transform {
                    translation: Vec3::new(x as f32, y as f32, 1.0),
                    ..default()
                },
            ));
        }
    }

    //Create Rooms
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
            commands.spawn(Room {
                rect: new_room.clone(),
            });
            rooms.push(new_room);
        }
    }
}

fn apply_map(
    mut set: ParamSet<(Query<(Entity, &mut Tile, &Transform)>, Query<&mut Tile>)>,
    query_room: Query<&Room>,
    tile_resolution: Res<TileResolution>,
    map: ResMut<Map>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    font_size: Res<FontSize>,
) {
    let text_style = create_text_style(
        &asset_server,
        &font_size,
        Srgba {
            red: 255.,
            green: 255.,
            blue: 0.,
            alpha: 0.4,
        },
    );

    let mut rng = rand::thread_rng();

    let mut i = 0;
    let (mut old_x, mut old_y) = (0, 0);
    for room in map.rooms.clone() {
        let new_room = query_room.get(room).unwrap().rect;
        apply_room_to_map(&new_room, &map, set.p1());
        if i > 0 {
            let (new_x, new_y) = new_room.center();
            if rng.gen_range(0..2) == 1 {
                apply_horizontal_tunnel(&map, old_x, new_x, old_y, set.p1(), &tile_resolution);
            } else {
                apply_horizontal_tunnel(&map, old_x, new_x, new_y, set.p1(), &tile_resolution);
                apply_vertical_tunnel(&map, old_y, new_y, old_x, set.p1(), &tile_resolution);
            }
        }
        (old_x, old_y) = new_room.center();
        i += 1;
    }

    for (ent, tile, transform) in set.p0().iter() {
        match tile.tiletype {
            TileType::Wall => {
                commands.entity(ent).insert(Text2dBundle {
                    text: Text::from_section(' ', text_style.clone())
                        .with_justify(JustifyText::Center),
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y,
                        1.0,
                    ),
                    ..default()
                });
            }
            TileType::Floor => {
                commands.entity(ent).insert(Text2dBundle {
                    text: Text::from_section(' ', text_style.clone())
                        .with_justify(JustifyText::Center),
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y,
                        1.0,
                    ),
                    ..default()
                });
            }
        }
    }
}

pub fn create_text_style(
    asset_server: &Res<AssetServer>,
    font_size: &Res<FontSize>,
    srgba: Srgba,
) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Mx437_IBM_BIOS.ttf"),
        font_size: font_size.0,
        color: bevy::color::Color::Srgba(srgba),
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
