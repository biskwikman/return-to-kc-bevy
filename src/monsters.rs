use crate::resources::Map;
use crate::{components::*, get_tile_idx};
use bevy::prelude::*;
use rand::Rng;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, add_monsters);
        app.add_systems(Update, set_occupied_tiles);
    }
}

fn set_occupied_tiles(
    map: Res<Map>,
    mut query_tiles: Query<&mut Tile>,
    mut query_monster: Query<(&mut Monster, &Position)>,
) {
    for mut tile in query_tiles.iter_mut() {
        tile.occupied = false;
    }
    for (mut monster, position) in query_monster.iter_mut() {
        let occupied_tile = map.tiles[get_tile_idx(position.x, position.y)];
        monster.occupied_tile = occupied_tile;
        query_tiles.get_mut(occupied_tile).unwrap().occupied = true;
    }
}

fn add_monsters(
    mut commands: Commands,
    query_rooms: Query<(Entity, &Room)>,
    query_transform: Query<&Transform>,
    mut query_tile: Query<&mut Tile>,
    map: Res<Map>,
) {
    for (_ent, room) in query_rooms.iter().skip(1) {
        let center_tile = room.rect.center();
        let tile_ent = map.tiles[get_tile_idx(center_tile.0 as usize, center_tile.1 as usize)];
        let tile_trans = query_transform.get(tile_ent).unwrap();
        let occupied_tile = map.tiles[get_tile_idx(center_tile.0 as usize, center_tile.1 as usize)];
        query_tile.get_mut(occupied_tile).unwrap().occupied = true;
        let mut rng = rand::thread_rng();
        let glyph: char;
        if rng.gen_range(0..2) == 1 {
            println!("goblin");
            glyph = 'g';
        } else {
            println!("ogre");
            glyph = 'o';
        }

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    glyph,
                    TextStyle {
                        font: map.font.clone(),
                        font_size: map.font_size,
                        color: Color::Srgba(Srgba {
                            red: 1.0,
                            green: 0.0,
                            blue: 0.0,
                            alpha: 0.0,
                        }),
                    },
                )
                .with_justify(JustifyText::Center),
                transform: Transform::from_xyz(
                    tile_trans.translation.x,
                    tile_trans.translation.y,
                    2.0,
                ),
                ..default()
            },
            Position {
                x: center_tile.0 as usize,
                y: center_tile.1 as usize,
            },
            Monster {
                visibility: VisibleType::Visible,
                occupied_tile,
            },
        ));
    }
}
