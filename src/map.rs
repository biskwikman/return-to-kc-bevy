use crate::components::*;
// use crate::rect::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn get_tile_idx(idx_xy: (usize, usize)) -> usize {
    idx_xy.0 + 80 * idx_xy.1
}

pub fn create_map(
    mut query: Query<(Entity, &mut Tile, &Transform)>,
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
    for (ent, _tile, _transform) in query.iter() {
        map.tiles.push(ent);
    }

    // query
    //     .get_mut(map.tiles[get_tile_idx((12, 12))])
    //     .unwrap()
    //     .1
    //     .tiletype = TileType::Wall;

    for (ent, tile, transform) in query.iter() {
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

pub fn apply_room_to_map(room: &Rect, mut map: ResMut<Map>, mut query: Query<&mut Tile>) {}
