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
}

#[derive(Component)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    // let width = x_range.len();

    for (iy, y) in (y_min as i32..y_max as i32)
        .step_by(font_size as usize)
        .enumerate()
    {
        for (ix, x) in x_range.clone().enumerate() {
            // let idx = ix + width * iy;
            commands.spawn((
                Tile {
                    zone: PlayerZone::Outside,
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
    (idx_xy.1 as i32 * 80 - 80 + idx_xy.0 as i32 - 1) as usize
}

fn create_map(query: Query<Entity, With<Tile>>, mut map: ResMut<Map>) {
    for ent in query.iter() {
        map.tiles.push(ent);
    }
}

fn set_player_zones(
    mut commands: Commands,
    map: ResMut<Map>,
    mut query_tiles: Query<&mut Tile>,
    mut query_player_pos: Query<&Position, With<Player>>,
) {
    let player_pos = query_player_pos.single_mut();
    println!("player pos {}, {}", player_pos.x, player_pos.y);
    println!("player pos {}", get_tile_idx((player_pos.x, player_pos.y)));

    query_tiles
        .get_mut(map.tiles[get_tile_idx((player_pos.x, player_pos.y))])
        .unwrap()
        .zone = PlayerZone::Player;

    if player_pos.x > 1 {
        query_tiles
            .get_mut(map.tiles[get_tile_idx((player_pos.x - 1, player_pos.y))])
            .unwrap()
            .zone = PlayerZone::PlayerLeft;
    }
    if player_pos.x < 80 {
        query_tiles
            .get_mut(map.tiles[get_tile_idx((player_pos.x + 1, player_pos.y))])
            .unwrap()
            .zone = PlayerZone::PlayerRight;
    }
    if player_pos.y > 1 {
        query_tiles
            .get_mut(map.tiles[get_tile_idx((player_pos.x, player_pos.y - 1))])
            .unwrap()
            .zone = PlayerZone::PlayerBottom;
    }
    if player_pos.y < 60 {
        query_tiles
            .get_mut(map.tiles[get_tile_idx((player_pos.x, player_pos.y + 1))])
            .unwrap()
            .zone = PlayerZone::PlayerTop;
    }

    println!(
        "PlayerLeft {}",
        get_tile_idx((player_pos.x - 1, player_pos.y))
    );
    println!(
        "PlayerRight {}",
        get_tile_idx((player_pos.x + 1, player_pos.y))
    );
    println!(
        "PlayerBottom {}",
        get_tile_idx((player_pos.x, player_pos.y - 1))
    );
}

fn add_player(
    mut commands: Commands,
    mut query_tiles: Query<(&mut Tile, &Position, &Transform)>,
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
    for (mut tile, position, transform) in &mut query_tiles {
        if position.y == player_spawn_y && position.x == player_spawn_x {
            // tile.zone = PlayerZone::Player;
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
        // else if position.y == player_spawn_y + 1 && position.x == player_spawn_x {
        //     tile.zone = PlayerZone::PlayerTop;
        // } else if position.y == player_spawn_y - 1 && position.x == player_spawn_x {
        //     tile.zone = PlayerZone::PlayerBottom;
        // } else if position.y == player_spawn_y && position.x == player_spawn_x - 1 {
        //     tile.zone = PlayerZone::PlayerLeft;
        // } else if position.y == player_spawn_y && position.x == player_spawn_x + 1 {
        //     tile.zone = PlayerZone::PlayerRight;
        // }
    }
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<(&mut Transform, &mut Position), With<Player>>,
    mut query_tiles: Query<(&mut Tile, &Position, &Transform), Without<Player>>,
) {
    let (mut player_transform, mut player_position) = query_player.single_mut();

    for (mut tile, tile_position, tile_transform) in query_tiles.iter_mut() {
        match tile.zone {
            PlayerZone::PlayerTop => {
                if keys.just_pressed(KeyCode::KeyK) {
                    player_transform.translation.y = tile_transform.translation.y;
                    player_position.y = tile_position.y;
                    tile.zone = PlayerZone::Player;
                    break;
                }
            }
            PlayerZone::PlayerBottom => {
                println!("bottom tile: {}, {}", tile_position.x, tile_position.y);
                if keys.just_pressed(KeyCode::KeyJ) {
                    player_transform.translation.y = tile_transform.translation.y;
                    player_position.y = tile_position.y;
                    tile.zone = PlayerZone::Player;
                    break;
                }
            }
            PlayerZone::PlayerLeft => {
                if keys.just_pressed(KeyCode::KeyH) {
                    player_transform.translation.x = tile_transform.translation.x;
                    player_position.x = tile_position.x;
                    tile.zone = PlayerZone::Player;
                    break;
                }
            }
            PlayerZone::PlayerRight => {
                if keys.just_pressed(KeyCode::KeyL) {
                    player_transform.translation.x = tile_transform.translation.x;
                    player_position.x = tile_position.x;
                    tile.zone = PlayerZone::Player;
                    break;
                }
            }
            _ => {}
        }
    }

    for (mut tile, _tile_position, _tile_transform) in query_tiles.iter_mut() {
        match tile.zone {
            PlayerZone::PlayerTop => tile.zone = PlayerZone::Outside,
            PlayerZone::PlayerBottom => tile.zone = PlayerZone::Outside,
            PlayerZone::PlayerLeft => tile.zone = PlayerZone::Outside,
            PlayerZone::PlayerRight => tile.zone = PlayerZone::Outside,
            _ => {}
        }
    }

    // for (mut tile, tile_position, _tile_transform) in query_tiles.iter_mut() {
    //     if tile_position.x == player_position.x && tile_position.y == player_position.y + 1 {
    //         tile.zone = PlayerZone::PlayerTop;
    //     }
    //     if tile_position.x == player_position.x && tile_position.y == player_position.y - 1 {
    //         tile.zone = PlayerZone::PlayerBottom;
    //     }
    //     if tile_position.x == player_position.x - 1 && tile_position.y == player_position.y {
    //         tile.zone = PlayerZone::PlayerLeft;
    //     }
    //     if tile_position.x == player_position.x + 1 && tile_position.y == player_position.y {
    //         tile.zone = PlayerZone::PlayerRight;
    //     }
    // }
}
