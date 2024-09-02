use crate::components::*;
// use crate::rect::*;
use crate::resources::*;
use bevy::prelude::*;

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

    let room1 = Rect::new(9., 9., 35., 25.);

    // apply_room_to_map(&room1, map, set.p1());

    for y in room1.min.y as usize + 1..=room1.max.y as usize {
        for x in room1.min.x as usize + 1..=room1.max.x as usize {
            set.p1()
                .get_mut(map.tiles[get_tile_idx((x, y))])
                .unwrap()
                .tiletype = TileType::Floor;
        }
    }

    // query
    //     .get_mut(map.tiles[get_tile_idx((12, 12))])
    //     .unwrap()
    //     .1
    //     .tiletype = TileType::Wall;

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

pub fn apply_room_to_map(room: &Rect, map: ResMut<Map>, mut query: Query<&mut Tile>) {
    for y in room.min.y as usize + 1..=room.max.y as usize {
        for x in room.min.x as usize + 1..=room.max.x as usize {
            query
                .get_mut(map.tiles[get_tile_idx((x, y))])
                .unwrap()
                .tiletype = TileType::Floor;
        }
    }
}
