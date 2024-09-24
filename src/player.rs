use crate::components::*;
use crate::events::*;
use crate::map::*;
use crate::resources::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostStartup,
            (add_player, set_player_zones.after(add_player)),
        );
    }
}

pub fn move_player(
    map: ResMut<Map>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<(&mut Transform, &mut Position), With<Player>>,
    mut query_tiles: Query<(&mut Tile, &Position, &Transform), Without<Player>>,
    mut events: EventWriter<Tick>,
) {
    let (mut player_transform, mut player_pos) = query_player.single_mut();

    if keys.just_pressed(KeyCode::KeyK) {
        let top_tile = if player_pos.y < 58 {
            map.tiles[get_tile_idx((player_pos.x, player_pos.y + 1))]
        } else {
            map.tiles[get_tile_idx((player_pos.x, 59))]
        };
        if query_tiles.get_mut(top_tile).unwrap().0.tiletype != TileType::Wall {
            player_transform.translation.y = query_tiles.get_mut(top_tile).unwrap().2.translation.y;
            player_pos.y = query_tiles.get_mut(top_tile).unwrap().1.y;
        }
        events.send(Tick);
    }
    if keys.just_pressed(KeyCode::KeyJ) {
        let bot_tile = if player_pos.y > 1 {
            map.tiles[get_tile_idx((player_pos.x, player_pos.y - 1))]
        } else {
            map.tiles[get_tile_idx((player_pos.x, 0))]
        };
        if query_tiles.get_mut(bot_tile).unwrap().0.tiletype != TileType::Wall {
            player_transform.translation.y = query_tiles.get_mut(bot_tile).unwrap().2.translation.y;
            player_pos.y = query_tiles.get_mut(bot_tile).unwrap().1.y;
        }
        events.send(Tick);
    }
    if keys.just_pressed(KeyCode::KeyH) {
        let left_tile = if player_pos.x > 1 {
            map.tiles[get_tile_idx((player_pos.x - 1, player_pos.y))]
        } else {
            map.tiles[get_tile_idx((0, player_pos.y))]
        };

        if query_tiles.get_mut(left_tile).unwrap().0.tiletype != TileType::Wall {
            player_transform.translation.x =
                query_tiles.get_mut(left_tile).unwrap().2.translation.x;
            player_pos.x = query_tiles.get_mut(left_tile).unwrap().1.x;
        }
        events.send(Tick);
    }
    if keys.just_pressed(KeyCode::KeyL) {
        let right_tile = if player_pos.x < 78 {
            map.tiles[get_tile_idx((player_pos.x + 1, player_pos.y))]
        } else {
            map.tiles[get_tile_idx((79, player_pos.y))]
        };
        if query_tiles.get_mut(right_tile).unwrap().0.tiletype != TileType::Wall {
            player_transform.translation.x =
                query_tiles.get_mut(right_tile).unwrap().2.translation.x;
            player_pos.x = query_tiles.get_mut(right_tile).unwrap().1.x;
        }
        events.send(Tick);
    }

    for (mut tile, _tile_position, _tile_transform) in query_tiles.iter_mut() {
        match tile.zone {
            PlayerZone::PlayerTop => tile.zone = PlayerZone::Outside,
            PlayerZone::PlayerBottom => tile.zone = PlayerZone::Outside,
            PlayerZone::PlayerLeft => tile.zone = PlayerZone::Outside,
            PlayerZone::PlayerRight => tile.zone = PlayerZone::Outside,
            PlayerZone::Player => tile.zone = PlayerZone::Outside,
            _ => {}
        }
    }
}

pub fn add_player(
    mut commands: Commands,
    mut query_tiles: Query<(&Position, &Transform)>,
    query_rooms: Query<(Entity, &Room)>,
    map: ResMut<Map>,
    asset_server: Res<AssetServer>,
) {
    let (player_spawn_x, player_spawn_y) = query_rooms.get(map.rooms[0]).unwrap().1.rect.center();
    let font = asset_server.load("fonts/Mx437_IBM_BIOS.ttf");
    let font_size = 10.0;
    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        ..default()
    };
    let text_justification = JustifyText::Center;
    for (position, transform) in &mut query_tiles {
        if position.y as i32 == player_spawn_y && position.x as i32 == player_spawn_x {
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section('@', text_style.clone())
                        .with_justify(text_justification),
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y,
                        1.0,
                    ),
                    ..default()
                },
                Player {},
                Position {
                    x: position.x,
                    y: position.y,
                },
                Viewshed {
                    visible_tiles: Vec::new(),
                    range: 6,
                },
            ));
        }
    }
}

pub fn set_player_zones(
    map: ResMut<Map>,
    mut query_tiles: Query<&mut Tile>,
    mut query_player_pos: Query<&Position, With<Player>>,
) {
    let player_pos = query_player_pos.single_mut();

    query_tiles
        .get_mut(map.tiles[get_tile_idx((player_pos.x, player_pos.y))])
        .unwrap()
        .zone = PlayerZone::Player;

    if player_pos.x > 0 {
        query_tiles
            .get_mut(map.tiles[get_tile_idx((player_pos.x - 1, player_pos.y))])
            .unwrap()
            .zone = PlayerZone::PlayerLeft;
    }
    if player_pos.x < 79 {
        query_tiles
            .get_mut(map.tiles[get_tile_idx((player_pos.x + 1, player_pos.y))])
            .unwrap()
            .zone = PlayerZone::PlayerRight;
    }
    if player_pos.y > 0 {
        query_tiles
            .get_mut(map.tiles[get_tile_idx((player_pos.x, player_pos.y - 1))])
            .unwrap()
            .zone = PlayerZone::PlayerBottom;
    }
    if player_pos.y < 59 {
        query_tiles
            .get_mut(map.tiles[get_tile_idx((player_pos.x, player_pos.y + 1))])
            .unwrap()
            .zone = PlayerZone::PlayerTop;
    }
}
