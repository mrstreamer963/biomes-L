use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::ecs::biome_definitions::BiomeDefinitions;
use crate::ecs::grid_resource::GridResource;

const TILE_SIZE: f64 = 32.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    col: u32,
    row: u32,
    f_score: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: (u32, u32), b: (u32, u32)) -> u32 {
    let dx = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    let dy = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
    dx + dy
}

/// Finds a path from `start` to `end` using A* on the grid.
/// Returns `None` if no path exists.
/// Only considers passable cells (biomes with `passable == true`).
pub fn find_path(
    grid: &GridResource,
    defs: &BiomeDefinitions,
    start: (u32, u32),
    end: (u32, u32),
) -> Option<Vec<(u32, u32)>> {
    let width = grid.width;
    let height = grid.height;

    // Bounds check
    if start.0 >= width || start.1 >= height || end.0 >= width || end.1 >= height {
        return None;
    }

    // Check if start or end is impassable
    let start_idx = (start.1 as usize) * (width as usize) + (start.0 as usize);
    let start_biome = grid.biome_ids.get(start_idx).copied().unwrap_or(0);
    if !defs.is_passable(start_biome) {
        return None;
    }

    let end_idx = (end.1 as usize) * (width as usize) + (end.0 as usize);
    let end_biome = grid.biome_ids.get(end_idx).copied().unwrap_or(0);
    if !defs.is_passable(end_biome) {
        return None;
    }

    // Early exit if start == end
    if start == end {
        return Some(vec![start]);
    }

    // A* main loop
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    let mut g_score: HashMap<(u32, u32), u32> = HashMap::new();

    let start_f = heuristic(start, end);
    open_set.push(Node {
        col: start.0,
        row: start.1,
        f_score: start_f,
    });
    g_score.insert(start, 0);

    // 4-directional neighbors
    let directions: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    while let Some(current) = open_set.pop() {
        let current_pos = (current.col, current.row);

        if current_pos == end {
            // Reconstruct path
            let mut path = Vec::new();
            let mut pos = end;
            loop {
                path.push(pos);
                if pos == start {
                    break;
                }
                pos = match came_from.get(&pos) {
                    Some(&p) => p,
                    None => break,
                };
            }
            path.reverse();
            return Some(path);
        }

        let current_g = g_score[&current_pos];

        for &(dc, dr) in &directions {
            let nc = current.col as i32 + dc;
            let nr = current.row as i32 + dr;

            if nc < 0 || nr < 0 {
                continue;
            }
            let nc = nc as u32;
            let nr = nr as u32;

            if nc >= width || nr >= height {
                continue;
            }

            // Check passability
            let nidx = (nr as usize) * (width as usize) + (nc as usize);
            let nbiome = grid.biome_ids.get(nidx).copied().unwrap_or(0);
            if !defs.is_passable(nbiome) {
                continue;
            }

            let neighbor = (nc, nr);
            let tentative_g = current_g + 1;

            if tentative_g < g_score.get(&neighbor).copied().unwrap_or(u32::MAX) {
                came_from.insert(neighbor, current_pos);
                g_score.insert(neighbor, tentative_g);
                let f = tentative_g + heuristic(neighbor, end);
                open_set.push(Node {
                    col: nc,
                    row: nr,
                    f_score: f,
                });
            }
        }
    }

    // No path found
    None
}

/// Converts pixel coordinates to tile coordinates.
pub fn pixel_to_tile(x: f64, y: f64) -> (u32, u32) {
    let col = (x / TILE_SIZE).floor().max(0.0) as u32;
    let row = (y / TILE_SIZE).floor().max(0.0) as u32;
    (col, row)
}

/// Converts tile coordinates to pixel center coordinates.
pub fn tile_to_pixel(col: u32, row: u32) -> (f64, f64) {
    (col as f64 * TILE_SIZE + 16.0, row as f64 * TILE_SIZE + 16.0)
}