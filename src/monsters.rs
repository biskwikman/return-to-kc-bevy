use crate::resources::Map;
use crate::{components::*, get_tile_idx};
use bevy::prelude::*;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, add_monsters);
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

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    'g',
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
                ..default()
            },
            Position {
                x: center_tile.0 as usize,
                y: center_tile.1 as usize,
            },
            Monster,
        ));
    }
}
