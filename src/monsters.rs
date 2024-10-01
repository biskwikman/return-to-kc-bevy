use crate::components::*;
use bevy::prelude::*;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, add_monsters);
    }
}

fn add_monsters(mut commands: Commands, query_rooms: Query<(Entity, &Room)>) {
    for (ent, room) in query_rooms.iter().skip(1) {
        commands.spawn(Text2dBundle {
            text: Text::from_section('g'),
        });
    }
}
