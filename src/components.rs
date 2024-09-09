use bevy::prelude::*;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Tile>,
    pub range: i32,
}

#[derive(Component)]
pub struct Room {
    pub rect: crate::rect::Rect,
}

#[derive(Component, Debug)]
pub struct Tile {
    pub zone: PlayerZone,
    pub tiletype: TileType,
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
