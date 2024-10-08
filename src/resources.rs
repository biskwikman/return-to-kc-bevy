use bevy::prelude::*;

#[derive(Resource)]
pub struct Map {
    pub tiles: Vec<Entity>,
    pub rooms: Vec<Entity>,
    pub font_size: f32,
    pub font: Handle<Font>,
    pub tile_res: TileResolution,
}

#[derive(Clone, Copy)]
pub struct TileResolution {
    pub height: usize,
    pub width: usize,
}

impl FromWorld for Map {
    fn from_world(world: &mut World) -> Self {
        let font = world.load_asset("fonts/Mx437_IBM_BIOS.ttf");
        let font_size = 10.0;
        let mut window_query = world.query::<&Window>();
        let window = window_query.single(world);
        let map_width = window.resolution.width() / font_size;
        let map_height = window.resolution.height() / font_size;

        Map {
            tiles: Vec::new(),
            rooms: Vec::new(),
            font_size,
            font: font.clone(),
            tile_res: TileResolution {
                height: map_height as usize,
                width: map_width as usize,
            },
        }
    }
}
