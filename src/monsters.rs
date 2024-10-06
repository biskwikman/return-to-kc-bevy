use crate::events::*;
use crate::resources::Map;
use crate::{components::*, get_tile_idx};
use bevy::prelude::*;
use rand::Rng;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, add_monsters);
        app.add_systems(
            Update,
            (
                // unset_occupied_tiles.before(set_occupied_tiles),
                set_occupied_tiles.run_if(on_event::<Tick>()),
                monster_ai.run_if(on_event::<Tick>()),
            ),
        );
    }
}

// fn unset_occupied_tiles(mut commands: Commands, query_occupied: Query<Entity, With<Occupied>>) {
//     for tile_ent in query_occupied.iter() {
//         commands.entity(tile_ent).remove::<Occupied>();
//     }
// }

fn set_occupied_tiles(
    map: Res<Map>,
    mut commands: Commands,
    mut query_monster: Query<(&mut Monster, &Position)>,
) {
    for (mut monster, position) in query_monster.iter_mut() {
        let occupied_tile = map.tiles[get_tile_idx(position.x, position.y)];
        monster.occupied_tile = occupied_tile;
        commands.entity(occupied_tile).insert(Occupied);
    }
}

fn add_monsters(
    mut commands: Commands,
    query_rooms: Query<(Entity, &Room)>,
    query_transform: Query<&Transform>,
    map: Res<Map>,
) {
    for (_ent, room) in query_rooms.iter().skip(1) {
        let center_tile = room.rect.center();
        let tile_ent = map.tiles[get_tile_idx(center_tile.0 as usize, center_tile.1 as usize)];
        let tile_trans = query_transform.get(tile_ent).unwrap();
        let occupied_tile = map.tiles[get_tile_idx(center_tile.0 as usize, center_tile.1 as usize)];
        commands.entity(occupied_tile).insert(Occupied);
        let mut rng = rand::thread_rng();
        let glyph: char;
        let name: String;

        if rng.gen_range(0..2) == 1 {
            name = "Goblin".to_string();
            glyph = 'g';
        } else {
            name = "Ogre".to_string();
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
                            alpha: 1.0,
                        }),
                    },
                )
                .with_justify(JustifyText::Center),
                transform: Transform::from_xyz(
                    tile_trans.translation.x,
                    tile_trans.translation.y,
                    2.0,
                ),
                visibility: Visibility::Hidden,
                ..default()
            },
            Position {
                x: center_tile.0 as usize,
                y: center_tile.1 as usize,
            },
            Monster { occupied_tile },
            Name::new(name),
        ));
    }
}

fn monster_ai(
    query_monsters: Query<(&Visibility, &Name, &Position), With<Monster>>,
    query_player: Query<&Position, With<Player>>,
) {
    let player_pos = query_player.get_single().unwrap();
    for (visibility, name, position) in query_monsters.iter() {
        match visibility {
            Visibility::Visible => {
                let angle =
                    get_angle(player_pos.x, player_pos.y, position.x, position.y).to_degrees();
                println!("{name} glares at you, unmoving, at an angle of {angle}.");
            }
            _ => {}
        }
    }
}

fn get_angle(
    player_pos_x: usize,
    player_pos_y: usize,
    monster_pos_x: usize,
    monster_pos_y: usize,
) -> f32 {
    let y = player_pos_y as f32 - monster_pos_y as f32;
    let x = player_pos_x as f32 - monster_pos_x as f32;
    y.atan2(x)
}
