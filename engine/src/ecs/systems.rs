use bevy_ecs::prelude::*;
use bevy_ecs::query::With;

use crate::ecs::components::*;
use crate::ecs::biome_definitions::BiomeDefinitions;
use crate::ecs::grid_resource::GridResource;
use crate::ecs::GameTime;

const TILE_SIZE: f64 = 32.0;

pub fn movement_system(
    time: Res<GameTime>,
    grid: Res<GridResource>,
    defs: Res<BiomeDefinitions>,
    mut query: Query<(&mut Position, &MovementTarget, &BaseSpeed, &mut MovementStatus, &mut Path), With<MovementTarget>>,
) {
    let dt = time.delta;
    for (mut pos, target, speed, mut status, mut path) in query.iter_mut() {
        let (tx, ty) = match target.0 {
            Some(t) => t,
            None => {
                status.idling = true;
                continue;
            }
        };

        // Determine current target: first waypoint in path, or the movement target directly
        let current_target = if !path.0.is_empty() {
            path.0[0]
        } else {
            (tx, ty)
        };

        let dx = current_target.0 - pos.x;
        let dy = current_target.1 - pos.y;
        let dist = (dx * dx + dy * dy).sqrt();

        // Arrived at current waypoint
        if dist < 2.0 {
            if !path.0.is_empty() {
                // Advance to next waypoint
                path.0.remove(0);
                if path.0.is_empty() {
                    // Path exhausted — we're at the final target
                    status.idling = true;
                } else {
                    status.idling = false;
                }
            } else {
                status.idling = true;
            }
            continue;
        }

        // Determine biome under the unit
        let col = (pos.x / TILE_SIZE).floor() as u32;
        let row = (pos.y / TILE_SIZE).floor() as u32;
        let idx = (row as usize) * (grid.width as usize) + (col as usize);
        let biome_id = grid.biome_ids.get(idx).copied().unwrap_or(0);
        let speed_factor = defs.definitions.get(biome_id as usize)
            .map(|d| d.speed_factor as f64)
            .unwrap_or(1.0);

        if speed_factor == 0.0 {
            // Impassable biome — stop (shouldn't happen with valid path, but safety check)
            status.idling = true;
            continue;
        }

        status.speed_multiplier = speed_factor;
        status.idling = false;

        let step = speed.0 * speed_factor * dt;
        let nx = pos.x + (dx / dist) * step;
        let ny = pos.y + (dy / dist) * step;

        // Check new biome for passability
        let ncol = (nx / TILE_SIZE).floor() as u32;
        let nrow = (ny / TILE_SIZE).floor() as u32;
        let nidx = (nrow as usize) * (grid.width as usize) + (ncol as usize);
        let nbiome = grid.biome_ids.get(nidx).copied().unwrap_or(0);
        let npassable = defs.definitions.get(nbiome as usize)
            .map(|d| d.passable)
            .unwrap_or(false);

        if !npassable {
            status.idling = true;
            continue;
        }

        // Clamp to map bounds
        let max_x = (grid.width as f64) * TILE_SIZE;
        let max_y = (grid.height as f64) * TILE_SIZE;
        pos.x = nx.clamp(0.0, max_x);
        pos.y = ny.clamp(0.0, max_y);
    }
}