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
        .add_systems(Update, animate_translation)
        .run();
}

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct Tile {
    column: usize,
    row: usize,
    center: (f32, f32),
    text: Option<Text2dBundle>,
}

// fn input(keys: Res<ButtonInput<KeyCode>>) {
//     if keys.just_pressed(KeyCode::KeyK) {}
// }

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut query: Query<&Window>) {
    let window = query.single();
    let font = asset_server.load("fonts/Mx437_IBM_BIOS.ttf");
    let font_size = 10.0;
    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        ..default()
    };

    let text_justification = JustifyText::Center;

    commands.spawn(Camera2dBundle::default());

    let y_max = window.resolution.height() / 2.0;
    let y_min = window.resolution.height() / -2.0 + font_size / 2.0;
    let x_max = window.resolution.width() / 2.0;
    let x_min = window.resolution.width() / -2.0 + font_size / 2.0;

    for (iy, y) in (y_min as i32..y_max as i32)
        .step_by(font_size as usize)
        .enumerate()
    {
        for (ix, x) in (x_min as i32..x_max as i32)
            .step_by(font_size as usize)
            .enumerate()
        {
            commands.spawn(Tile {
                center: (x as f32, y as f32),
                column: iy,
                row: ix,
            },
            commands.spawn(Text2dBundle {
                text: Text::from_section('@', text_style.clone()).with_justify(text_justification),
                transform: Transform::from_xyz(x as f32, y as f32, 1.0),
                ..default()
            });
        }
    }
}

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    // for mut transform in &mut query {
    //     transform.translation.x = 100.0 * time.elapsed_seconds().sin() - 400.0;
    //     transform.translation.y = 100.0 * time.elapsed_seconds().cos();
    // }
}
