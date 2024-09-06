use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Map {
    pub tiles: Vec<Entity>,
}

#[derive(Resource, Default, Clone, Copy)]
pub struct TileResolution {
    pub height: usize,
    pub width: usize,
}

#[derive(Resource)]
pub struct FontSize(pub f32);
