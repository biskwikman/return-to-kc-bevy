use bevy::{prelude::*, window::WindowResolution};
mod components;
use components::*;
mod player;
use player::*;
mod resources;
use resources::*;
mod map;
use map::*;
mod rect;

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
