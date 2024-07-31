use bevy::{
    color::palettes::css::*,
    prelude::*,
    reflect::DynamicTypePath,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds},
    window::WindowResolution,
};

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
        .add_systems(Startup, setup)
        .add_systems(PostStartup, add_player)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component, Debug)]
struct Tile {
    y: usize,
    x: usize,
    center: (f32, f32),
    idx: usize,
    player: PlayerZone,
}

#[derive(Debug, PartialEq)]
enum PlayerZone {
    Player,
    PlayerTop,
    PlayerBottom,
    PlayerLeft,
    PlayerRight,
    Outside,
}

#[derive(Component)]
struct Player {
    y: usize,
    x: usize,
    center: (f32, f32),
}

fn setup(mut commands: Commands, query_window: Query<&Window>) {
    let window = query_window.single();
    let font_size = 10.0;

    commands.spawn(Camera2dBundle::default());

    let y_max = window.resolution.height() / 2.0;
    let y_min = window.resolution.height() / -2.0 + font_size / 2.0;
    let x_max = window.resolution.width() / 2.0;
    let x_min = window.resolution.width() / -2.0 + font_size / 2.0;
    let x_range = (x_min as i32..x_max as i32).step_by(font_size as usize);
    let width = x_range.len();

    for (iy, y) in (y_min as i32..y_max as i32)
        .step_by(font_size as usize)
        .enumerate()
    {
        for (ix, x) in x_range.clone().enumerate() {
            let idx = ix + width * iy;
            commands.spawn(Tile {
                center: (x as f32, y as f32),
                y: iy,
                x: ix,
                idx,
                player: PlayerZone::Outside,
            });
        }
    }
}

fn add_player(
    mut commands: Commands,
    mut query_tiles: Query<&mut Tile>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Mx437_IBM_BIOS.ttf");
    let font_size = 10.0;
    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        ..default()
    };
    let text_justification = JustifyText::Center;
    for mut tile in &mut query_tiles {
        if tile.y == 30 && tile.x == 20 {
            tile.player = PlayerZone::Player;
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section('@', text_style.clone())
                        .with_justify(text_justification),
                    transform: Transform::from_xyz(tile.center.0, tile.center.1, 1.0),
                    ..default()
                },
                Player {
                    y: tile.y,
                    x: tile.x,
                    center: tile.center,
                },
            ));
        } else if tile.y == 30 && tile.x == 21 {
            tile.player = PlayerZone::PlayerTop
        }
    }
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<(&mut Transform, &mut Player)>,
    mut query_tiles: Query<&mut Tile>,
) {
    let (mut player_transform, mut player) = query_player.single_mut();
    println!("player y {}", player.y);

    for tile in query_tiles.iter() {
        if tile.x == player.x && tile.y == player.y + 1 {
            println!("tile y {}", tile.y);
            if keys.just_pressed(KeyCode::KeyK) {
                player_transform.translation.y = tile.center.1;
                player.y = tile.y;
                break;
            }
        }
    }

    for mut tile in query_tiles.iter_mut() {
        if tile.x == player.x && tile.y == player.y {
            tile.player = PlayerZone::Player;
        } else if tile.x == player.x && tile.y == player.y + 1 {
            tile.player = PlayerZone::PlayerTop;
        }
    }
}
