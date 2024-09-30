use bevy::{prelude::*, window::WindowResolution};

#[derive(Resource)]
pub struct Map {
    pub tiles: Vec<Entity>,
    pub rooms: Vec<Entity>,
    pub font_size: f32,
    pub tile_res: TileResolution,
}

// #[derive(Resource, Default, Clone, Copy)]
#[derive(Clone, Copy)]
pub struct TileResolution {
    pub height: usize,
    pub width: usize,
}

// #[derive(Resource)]
// pub struct FontSize(pub f32);

impl FromWorld for Map {
    fn from_world(world: &mut World) -> Self {
        let font_size = 10.0;
        let mut window_query = world.query::<&Window>();
        let window = window_query.single(world);
        let map_width = window.resolution.width() / font_size;
        let map_height = window.resolution.height() / font_size;

        Map {
            tiles: Vec::new(),
            rooms: Vec::new(),
            font_size,
            tile_res: TileResolution {
                height: map_height as usize,
                width: map_width as usize,
            },
        }
    }
}
