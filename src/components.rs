use bevy::prelude::*;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Entity>,
    pub range: i32,
}

#[derive(Component, Copy, Clone)]
pub struct Room {
    pub rect: crate::rect::Rect,
}

#[derive(Component, Debug)]
pub struct Tile {
    pub tiletype: TileType,
    pub visibletype: VisibleType,
    pub blocked: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VisibleType {
    Visible,
    Memoried,
    Invisible,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Monster {
    pub occupied_tile: Entity,
}

#[derive(Component)]
pub struct Occupied;
