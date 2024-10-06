use crate::components::*;
use crate::events::*;
use crate::get_tile_idx;
use crate::move_player;
use crate::resources::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::iter::zip;

pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                get_viewshed.run_if(on_event::<Tick>()),
                apply_view.after(get_viewshed).run_if(on_event::<Tick>()),
            )
                .after(move_player),
        );
    }
}

pub fn get_viewshed(
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

    let player_tile_ent = map.tiles[get_tile_idx(player_pos.x, player_pos.y)];

    for (mut tile, _pos, _trans) in set.p0().iter_mut() {
        match tile.visibletype {
            VisibleType::Visible => {
                tile.visibletype = VisibleType::Memoried;
            }
            VisibleType::Memoried => {}
            _ => {}
        }
    }

    for deg in (1..=360).step_by(1) {
        let radian = (deg as f32).to_radians();
        let radius = 60.0;
        let x = radius * radian.cos();
        let y = radius * radian.sin();

        let visible_tiles = cast_ray(
            &rapier_context,
            player_transform_x,
            player_transform_y,
            x,
            y,
            player_tile_ent,
            set.p1(),
        );

        for tile in visible_tiles {
            set.p1().get_mut(tile).unwrap().0.visibletype = VisibleType::Visible;
            player_viewshed.visible_tiles.push(tile);
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
) -> Vec<Entity> {
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
    let filter2 = QueryFilter::exclude_rigid_body(n, player_tile_ent);

    let mut intersected_tiles: Vec<Entity> = Vec::new();
    let mut tile_intersections: Vec<RayIntersection> = Vec::new();

    rapier_context.intersections_with_ray(
        ray_origin,
        ray_dir,
        max_toi,
        solid,
        filter2,
        |entity, intersection| {
            intersected_tiles.push(entity);
            tile_intersections.push(intersection);
            true
        },
    );

    let mut visible_tiles: Vec<Entity> = Vec::new();
    let nearest_wall: Option<(Entity, f32)>;
    // cast for nearest wall
    if let Some((entity, toi)) =
        rapier_context.cast_ray(ray_origin, ray_dir, max_toi, solid, filter)
    {
        nearest_wall = Some((entity, toi));
        // add nearest wall to visible tiles
        if let Some(wall) = nearest_wall {
            visible_tiles.push(wall.0);
        }

        for (ent, intersect) in zip(intersected_tiles.clone(), tile_intersections) {
            if custom_query.get(ent).unwrap().0.tiletype != TileType::Wall
                && intersect.time_of_impact < nearest_wall.unwrap().1
            {
                visible_tiles.push(ent);
            }
        }
    } else {
        visible_tiles.append(&mut intersected_tiles);
    }

    visible_tiles
}

pub fn apply_view(
    mut query_monsters: Query<(&Monster, &mut Visibility)>,
    mut query_occupied: Query<(&mut Tile, &mut Text), With<Occupied>>,
    mut query_unoccupied: Query<(&mut Tile, &mut Text), Without<Occupied>>,
    query_player: Query<&Viewshed, With<Player>>,
) {
    for (tile, mut text) in query_occupied.iter_mut() {
        match tile.visibletype {
            VisibleType::Visible => {
                if tile.tiletype == TileType::Floor {
                    text.sections[0].style.color = Color::srgba(0.0, 1.0, 0.0, 0.0);
                }
            }
            VisibleType::Memoried => {
                if tile.tiletype == TileType::Floor {
                    text.sections[0].style.color = Color::srgba(1.0, 1.0, 1.0, 0.5);
                }
            }
            _ => {}
        }
    }

    for (tile, mut text) in query_unoccupied.iter_mut() {
        match tile.visibletype {
            VisibleType::Visible => {
                if tile.tiletype == TileType::Floor {
                    text.sections[0].style.color = Color::srgba(0.0, 1.0, 0.0, 1.0);
                } else if tile.tiletype == TileType::Wall {
                    text.sections[0].style.color = Color::srgba(0.0, 1.0, 0.0, 1.0);
                }
            }
            VisibleType::Memoried => {
                if tile.tiletype == TileType::Floor {
                    text.sections[0].style.color = Color::srgba(1.0, 1.0, 1.0, 0.5);
                } else if tile.tiletype == TileType::Wall {
                    text.sections[0].style.color = Color::srgba(1.0, 1.0, 1.0, 0.5);
                }
            }
            _ => {}
        }
    }

    let player_viewshed = query_player.single();
    for (monster, mut visibility) in query_monsters.iter_mut() {
        if player_viewshed
            .visible_tiles
            .contains(&monster.occupied_tile)
        {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
