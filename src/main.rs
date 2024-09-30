use bevy::{prelude::*, window::WindowResolution};
mod components;
mod player;
use bevy_rapier2d::prelude::*;
use player::*;
mod resources;
use resources::*;
mod map;
use map::*;
mod rect;
mod visibility;
use visibility::*;
mod events;
use events::*;

fn main() {
    let window_resolution = WindowResolution::new(800.0, 600.0);
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: window_resolution.clone(),
            ..default()
        }),
        ..default()
    });
    App::new()
        .add_event::<Tick>()
        .add_plugins(default_plugins)
        .add_plugins((PlayerPlugin, MapPlugin, VisibilityPlugin))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .init_resource::<Map>()
        .add_systems(PreStartup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
