use bevy::{prelude::*, window::WindowResolution};
mod components;
mod player;
use player::*;
mod resources;
use resources::*;
mod map;
use map::*;
mod rect;
mod visibility;
use visibility::*;

fn main() {
    let window_resolution = WindowResolution::new(800.0, 600.0);
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: window_resolution.clone(),
            ..default()
        }),
        ..default()
    });
    let font_size = 10.0;
    App::new()
        .add_plugins(default_plugins)
        .add_plugins((PlayerPlugin, MapPlugin, VisibilityPlugin))
        .init_resource::<Map>()
        .insert_resource(FontSize(font_size))
        .insert_resource(TileResolution {
            width: (window_resolution.width() / font_size) as usize,
            height: (window_resolution.height() / font_size) as usize,
        })
        .add_systems(PreStartup, spawn_camera)
        .add_systems(Update, (move_player, set_player_zones.after(move_player)))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
