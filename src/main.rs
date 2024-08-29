use bevy::{prelude::*, window::WindowResolution};

fn main() {
    let window_resolution = WindowResolution::new(800.0, 600.0);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: window_resolution,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Map>()
        .add_systems(PreStartup, setup)
        .add_systems(Startup, create_map)
        .add_systems(
            PostStartup,
            (add_player, set_player_zones.after(add_player)),
        )
        .add_systems(Update, (move_player, set_player_zones.after(move_player)))
        .run();
}

#[derive(Resource, Default)]
struct Map {
    tiles: Vec<Entity>,
}

#[derive(Component, Debug)]
struct Tile {
    zone: PlayerZone,
    tiletype: TileType,
}

#[derive(Component)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TileType {
    Wall,
    Floor,
}

#[derive(Clone, Copy, Debug)]
enum PlayerZone {
    Player,
    PlayerTop,
    PlayerBottom,
    PlayerLeft,
    PlayerRight,
    Outside,
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, query_window: Query<&Window>) {
    let window = query_window.single();
    let font_size = 10.0;

    commands.spawn(Camera2dBundle::default());

    let y_max = window.resolution.height() / 2.0;
    let y_min = window.resolution.height() / -2.0 + font_size / 2.0;
    let x_max = window.resolution.width() / 2.0;
    let x_min = window.resolution.width() / -2.0 + font_size / 2.0;
    let x_range = (x_min as i32..x_max as i32).step_by(font_size as usize);

    for (iy, y) in (y_min as i32..y_max as i32)
        .step_by(font_size as usize)
        .enumerate()
    {
        for (ix, x) in x_range.clone().enumerate() {
            commands.spawn((
                Tile {
                    zone: PlayerZone::Outside,
                    tiletype: TileType::Floor,
                },
                Position { x: ix, y: iy },
                Transform {
                    translation: Vec3::new(x as f32, y as f32, 1.0),
                    ..default()
                },
            ));
        }
    }
}

fn get_tile_idx(idx_xy: (usize, usize)) -> usize {
    idx_xy.0 + 80 * idx_xy.1
}

fn create_map(
    mut query: Query<(Entity, &mut Tile, &Transform)>,
    // mut tile_query: Query<&mut Tile>,
    mut map: ResMut<Map>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Mx437_IBM_BIOS.ttf");
    let font_size = 10.0;
    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        ..default()
    };
    for (ent, _tile, _transform) in query.iter() {
        map.tiles.push(ent);
    }

    query
        .get_mut(map.tiles[get_tile_idx((12, 12))])
        .unwrap()
        .1
        .tiletype = TileType::Wall;

    for (ent, tile, transform) in query.iter() {
        match tile.tiletype {
            TileType::Wall => {
                commands.entity(ent).insert(Text2dBundle {
                    text: Text::from_section('#', text_style.clone())
                        .with_justify(JustifyText::Center),
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y,
                        1.0,
                    ),
                    ..default()
                });
            }
            TileType::Floor => {}
        }
    }
}

fn set_player_zones(
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

fn add_player(
    mut commands: Commands,
    mut query_tiles: Query<(&Position, &Transform)>,
    asset_server: Res<AssetServer>,
) {
    let player_spawn_x = 10;
    let player_spawn_y = 10;
    let font = asset_server.load("fonts/Mx437_IBM_BIOS.ttf");
    let font_size = 10.0;
    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        ..default()
    };
    let text_justification = JustifyText::Center;
    for (position, transform) in &mut query_tiles {
        if position.y == player_spawn_y && position.x == player_spawn_x {
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
            ));
        }
    }
}

fn move_player(
    map: ResMut<Map>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<(&mut Transform, &mut Position), With<Player>>,
    mut query_tiles: Query<(&mut Tile, &Position, &Transform), Without<Player>>,
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
        };
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
