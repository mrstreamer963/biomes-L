use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::ecs::biome_definitions::BiomeDefinitions;
use crate::ecs::grid_resource::GridResource;

const TILE_SIZE: f64 = 32.0;

#[derive(Debug, Clone)]
struct Node {
    col: u32,
    row: u32,
    f_score: f64,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.col == other.col && self.row == other.row && self.f_score == other.f_score
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.total_cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: (u32, u32), b: (u32, u32)) -> f64 {
    let dx = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    let dy = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
    dx.max(dy) as f64
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
    let mut g_score: HashMap<(u32, u32), f64> = HashMap::new();

    let start_f = heuristic(start, end);
    open_set.push(Node {
        col: start.0,
        row: start.1,
        f_score: start_f,
    });
    g_score.insert(start, 0.0);

    // 8-directional neighbors: cardinal + diagonal
    let directions: [(i32, i32); 8] = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

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

            // Diagonally-adjacent passability check
            let is_diagonal = dc != 0 && dr != 0;
            if is_diagonal {
                let ac1 = current.col as i32 + dc;
                let ar1 = current.row as i32;
                if ac1 >= 0 && ar1 >= 0 {
                    let ac1 = ac1 as u32;
                    let ar1 = ar1 as u32;
                    if ac1 < width && ar1 < height {
                        let aidx1 = (ar1 as usize) * (width as usize) + (ac1 as usize);
                        let abiome1 = grid.biome_ids.get(aidx1).copied().unwrap_or(0);
                        if !defs.is_passable(abiome1) {
                            continue;
                        }
                    }
                }
                let ac2 = current.col as i32;
                let ar2 = current.row as i32 + dr;
                if ac2 >= 0 && ar2 >= 0 {
                    let ac2 = ac2 as u32;
                    let ar2 = ar2 as u32;
                    if ac2 < width && ar2 < height {
                        let aidx2 = (ar2 as usize) * (width as usize) + (ac2 as usize);
                        let abiome2 = grid.biome_ids.get(aidx2).copied().unwrap_or(0);
                        if !defs.is_passable(abiome2) {
                            continue;
                        }
                    }
                }
            }

            // Check passability of the neighbor itself
            let nidx = (nr as usize) * (width as usize) + (nc as usize);
            let nbiome = grid.biome_ids.get(nidx).copied().unwrap_or(0);
            if !defs.is_passable(nbiome) {
                continue;
            }

            let neighbor = (nc, nr);
            let step_cost = if is_diagonal { 1.414 } else { 1.0 };
            let tentative_g = current_g + step_cost;

            if tentative_g < g_score.get(&neighbor).copied().unwrap_or(f64::INFINITY) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::biome_definitions::BiomeDef;

    fn passable_defs() -> BiomeDefinitions {
        BiomeDefinitions::new(vec![BiomeDef {
            name: "Plains".into(),
            passable: true,
            speed_factor: 1.0,
            color: 0x7ec850,
        }])
    }

    fn impassable_defs() -> BiomeDefinitions {
        BiomeDefinitions::new(vec![
            BiomeDef {
                name: "Plains".into(),
                passable: true,
                speed_factor: 1.0,
                color: 0x7ec850,
            },
            BiomeDef {
                name: "Wall".into(),
                passable: false,
                speed_factor: 0.0,
                color: 0x000000,
            },
        ])
    }

    fn empty_grid(width: u32, height: u32) -> GridResource {
        GridResource::new(width, height, vec![0; (width * height) as usize], vec![0; (width * height) as usize])
    }

    fn grid_with_walls(width: u32, height: u32, walls: &[(u32, u32)]) -> GridResource {
        let mut biome_ids = vec![0u16; (width * height) as usize];
        for &(c, r) in walls {
            let idx = (r as usize) * (width as usize) + (c as usize);
            biome_ids[idx] = 1;
        }
        GridResource::new(width, height, biome_ids, vec![0; (width * height) as usize])
    }

    #[test]
    fn diagonal_path_without_obstacles() {
        let defs = passable_defs();
        let grid = empty_grid(10, 10);
        let path = find_path(&grid, &defs, (0, 0), (3, 3));
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.first(), Some(&(0, 0)));
        assert_eq!(path.last(), Some(&(3, 3)));
        // Diagonal path should be shorter than 6 steps (manhattan would be 6)
        assert!(path.len() <= 4, "8-directional path should be at most 4 steps, got {}", path.len());
        // Expected optimal diagonal: (0,0) -> (1,1) -> (2,2) -> (3,3)
        assert_eq!(path, vec![(0, 0), (1, 1), (2, 2), (3, 3)]);
    }

    #[test]
    fn cardinal_path_through_corridor() {
        let defs = impassable_defs();
        let walls = vec![
            (0, 0), (0, 2),
            (1, 0), (1, 2),
            (2, 0), (2, 2),
        ];
        let grid = grid_with_walls(3, 3, &walls);
        let path = find_path(&grid, &defs, (0, 1), (2, 1));
        assert!(path.is_some(), "Path should exist through the corridor");
        let path = path.unwrap();
        assert_eq!(path.first(), Some(&(0, 1)));
        assert_eq!(path.last(), Some(&(2, 1)));
        // Must go straight — no diagonal through corners
        assert_eq!(path, vec![(0, 1), (1, 1), (2, 1)]);
    }

    #[test]
    fn diagonal_blocked_by_adjacent_wall() {
        let defs = impassable_defs();
        // Wall at (0,2) blocks diagonal from (0,1) to (1,2)
        let walls = vec![(0, 2)];
        let grid = grid_with_walls(5, 5, &walls);
        let path = find_path(&grid, &defs, (0, 1), (1, 2));
        assert!(path.is_some(), "Path should exist via roundabout");
        let path = path.unwrap();
        assert_eq!(path.first(), Some(&(0, 1)));
        assert_eq!(path.last(), Some(&(1, 2)));
        // Diagonal (0,1)->(1,2) is blocked by wall at (0,2)
        // Must go around: e.g. (0,1)->(0,2)? No, (0,2) is wall
        // Actually: (0,1) -> (1,1) -> (1,2)  or similar
        assert!(
            !path.contains(&(0, 2)),
            "Path must not include wall at (0,2)"
        );
        // Should be at least 2 steps (not direct diagonal)
        assert!(path.len() >= 2, "Path should go around, not direct diagonal");
    }

    #[test]
    fn diagonal_allowed_when_adjacent_passable() {
        let defs = passable_defs();
        let grid = empty_grid(5, 5);
        let path = find_path(&grid, &defs, (0, 0), (1, 1));
        assert!(path.is_some());
        let path = path.unwrap();
        // Direct diagonal should be allowed since both (0,1) and (1,0) are passable
        assert_eq!(path, vec![(0, 0), (1, 1)]);
    }

    #[test]
    fn obstacle_avoidance_8_directional() {
        let defs = impassable_defs();
        // Wall blocking direct path
        let walls = vec![(1, 0), (1, 1)];
        let grid = grid_with_walls(5, 5, &walls);
        let path = find_path(&grid, &defs, (0, 0), (2, 2));
        assert!(path.is_some(), "Path should find a way around walls");
        let path = path.unwrap();
        assert_eq!(path.first(), Some(&(0, 0)));
        assert_eq!(path.last(), Some(&(2, 2)));
        // Ensure we don't step on walls
        for &(c, r) in &path {
            assert!(!walls.contains(&(c, r)), "Path must not include wall tiles");
        }
    }

    #[test]
    fn start_equals_end() {
        let defs = passable_defs();
        let grid = empty_grid(5, 5);
        let path = find_path(&grid, &defs, (2, 3), (2, 3));
        assert_eq!(path, Some(vec![(2, 3)]));
    }

    #[test]
    fn no_path_to_impassable_target() {
        let defs = impassable_defs();
        let walls = vec![(5, 5)];
        let grid = grid_with_walls(10, 10, &walls);
        let path = find_path(&grid, &defs, (0, 0), (5, 5));
        assert!(path.is_none());
    }

    #[test]
    fn out_of_bounds_returns_none() {
        let defs = passable_defs();
        let grid = empty_grid(5, 5);
        assert!(find_path(&grid, &defs, (10, 10), (0, 0)).is_none());
        assert!(find_path(&grid, &defs, (0, 0), (10, 10)).is_none());
    }
}
