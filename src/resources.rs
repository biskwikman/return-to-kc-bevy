use bevy::prelude::*;
#[derive(Resource, Default)]
pub struct Map {
    pub tiles: Vec<Entity>,
}
