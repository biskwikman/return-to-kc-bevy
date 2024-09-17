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
    pub zone: PlayerZone,
    pub tiletype: TileType,
    pub visibletype: VisibleType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VisibleType {
    Visible,
    Obscured,
    Memoried,
    Undiscovered,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Clone, Copy, Debug)]
pub enum PlayerZone {
    Player,
    PlayerTop,
    PlayerBottom,
    PlayerLeft,
    PlayerRight,
    Outside,
}

#[derive(Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct Player;
