use crate::components::*;
use crate::create_text_style;
use crate::get_tile_idx;
use crate::resources::*;
use bevy::prelude::*;
use bevy::reflect::List;
use bevy::utils::tracing::instrument::WithSubscriber;

pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (get_viewshed, apply_view.after(get_viewshed)));
    }
}

fn get_viewshed(
    mut query_tiles: Query<Entity, With<Tile>>,
    mut query_player: Query<(&Position, &mut Viewshed), With<Player>>,
    map: Res<Map>,
) {
    let (player_pos, mut player_viewshed) = query_player.get_single_mut().unwrap();
    let view_range = player_viewshed.range;

    println!("NEW");
    let mut i_x: i32 = 1;
    let mut halfway = false;
    for y_range in (view_range * -1)..=view_range {
        println!("y_range {y_range}");
        let y = player_pos.y as i32 + y_range;
        // for x_range in (view_range * -1)..=view_range {
        for x_i2 in 1..=i_x {
            println!("i_x: {i_x}");
            let x_offset = (i_x / 2) - (i_x - 1);
            let x = player_pos.x as i32 + x_offset;
            let vis_tile_ent = query_tiles
                .get_mut(map.tiles[get_tile_idx((x as usize, y as usize))])
                .unwrap();
            player_viewshed.visible_tiles.push(vis_tile_ent);
        }
        if i_x < ((view_range * 2) + 1) && halfway == false {
            i_x += 2;
            println!("add");
        } else if i_x == ((view_range * 2) + 1) && halfway == false {
            halfway = true;
        } else if halfway == true {
            i_x -= 2;
            println!("sub");
        }
        // }
    }
}

fn apply_view(
    mut query_viewshed: Query<&mut Viewshed>,
    mut query_text: Query<&mut Text>,
    asset_server: Res<AssetServer>,
    font_size: Res<FontSize>,
) {
    let text_style = create_text_style(asset_server, font_size);

    let viewshed = query_viewshed.get_single_mut().unwrap();
    println!("{:?}", viewshed.visible_tiles.len());
    let mut iter = query_text.iter_many_mut(&viewshed.visible_tiles);
    while let Some(mut text) = iter.fetch_next() {
        text.sections = vec![TextSection::new('!', text_style.clone()); 1];
    }
}
