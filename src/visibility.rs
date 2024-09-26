use crate::components::*;
use crate::create_text_style;
use crate::events::*;
use crate::get_tile_idx;
use crate::resources::*;
use bevy::prelude::*;
pub struct VisibilityPlugin;
use bevy_rapier2d::prelude::*;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                get_viewshed.run_if(on_event::<Tick>()),
                apply_view.after(get_viewshed),
            ),
        );
    }
}

fn get_viewshed(
    rapier_context: Res<RapierContext>,
    mut set: ParamSet<(
        Query<(&mut Tile, &Position, &Transform)>,
        Query<(&mut Tile, &Position)>,
    )>,
    mut query_player: Query<(&Position, &Transform, &mut Viewshed), With<Player>>,
    map: Res<Map>,
) {
    let (player_pos, player_transform, mut player_viewshed) =
        query_player.get_single_mut().unwrap();
    player_viewshed.visible_tiles.clear();
    let player_transform_x = player_transform.translation.x;
    let player_transform_y = player_transform.translation.y;

    let player_tile_ent = map.tiles[get_tile_idx((player_pos.x, player_pos.y))];

    // FOV direction crawl
    for (mut tile, _pos, _trans) in set.p0().iter_mut() {
        match tile.visibletype {
            VisibleType::Visible => {
                tile.visibletype = VisibleType::Memoried;
            }
            VisibleType::Memoried => {}
            VisibleType::Obscured => {
                tile.visibletype = VisibleType::Undiscovered;
            }

            _ => {}
        }
    }

    for deg in (1..=360).step_by(1) {
        let radian = (deg as f32).to_radians();
        let radius = 60.0;
        let x = radius * radian.cos();
        let y = radius * radian.sin();

        let seen_tile = cast_ray(
            &rapier_context,
            player_transform_x,
            player_transform_y,
            x,
            y,
            player_tile_ent,
            set.p1(),
        );

        match seen_tile {
            Some(..) => {
                set.p1().get_mut(seen_tile.unwrap()).unwrap().0.visibletype = VisibleType::Visible;
            }
            None => {}
        }
    }
}

fn cast_ray(
    rapier_context: &Res<RapierContext>,
    player_x: f32,
    player_y: f32,
    tile_x: f32,
    tile_y: f32,
    player_tile_ent: Entity,
    custom_query: Query<(&mut Tile, &Position)>,
) -> Option<Entity> {
    let ray_origin = Vec2::new(player_x, player_y);
    let ray_dir = Vec2::new(tile_x, tile_y);
    let max_toi = 1.0;
    let solid = false;
    let n = QueryFilter::new();
    let predicate = |handle| {
        custom_query
            .get(handle)
            .is_ok_and(|tile| tile.0.tiletype == TileType::Wall)
    };
    let filter = QueryFilter::exclude_rigid_body(n, player_tile_ent).predicate(&predicate);

    if let Some((entity, _toi)) =
        rapier_context.cast_ray(ray_origin, ray_dir, max_toi, solid, filter)
    {
        Some(entity)
    } else {
        None
    }
}

fn apply_view(
    mut query_text: Query<(&mut Text, &Tile)>,
    asset_server: Res<AssetServer>,
    font_size: Res<FontSize>,
) {
    let text_style_vis = create_text_style(
        &asset_server,
        &font_size,
        Srgba {
            red: 60.,
            green: 170.,
            blue: 70.,
            alpha: 1.,
        },
    );
    let text_style_mem = create_text_style(
        &asset_server,
        &font_size,
        Srgba {
            red: 255.,
            green: 255.,
            blue: 255.,
            alpha: 0.2,
        },
    );

    for (mut text, tile) in query_text.iter_mut() {
        match tile.visibletype {
            VisibleType::Visible => {
                if tile.tiletype == TileType::Floor {
                    text.sections = vec![TextSection::new('.', text_style_vis.clone()); 1];
                } else {
                    text.sections = vec![TextSection::new('#', text_style_vis.clone()); 1];
                }
            }
            VisibleType::Obscured => {
                text.sections = vec![TextSection::new(' ', text_style_vis.clone()); 1]
            }
            VisibleType::Memoried => {
                if tile.tiletype == TileType::Floor {
                    text.sections = vec![TextSection::new('.', text_style_mem.clone()); 1];
                } else {
                    text.sections = vec![TextSection::new('#', text_style_mem.clone()); 1];
                }
            }
            _ => {}
        }
    }
}
