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

/// Returns all tile indices touched by the line segment from `from` to `to`.
/// Uses pixel-level sampling so no crossed cell is missed.
pub fn cells_on_line(from: (f64, f64), to: (f64, f64), width: u32, height: u32) -> Vec<(u32, u32)> {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let dist = dx.hypot(dy);
    let steps = dist.ceil().max(1.0) as u32;

    let mut cells = Vec::new();
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let x = from.0 + dx * t;
        let y = from.1 + dy * t;
        let col = (x / TILE_SIZE).floor() as i32;
        let row = (y / TILE_SIZE).floor() as i32;
        if col >= 0 && row >= 0 && (col as u32) < width && (row as u32) < height {
            let cell = (col as u32, row as u32);
            if cells.last() != Some(&cell) {
                cells.push(cell);
            }
        }
    }
    cells
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

fn cross(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

/// Returns the portal vertices (left, right) for the edge between cell (c1, r1)
/// and its neighbor in direction (dc, dr).
pub fn portal_vertices(
    c1: u32,
    r1: u32,
    dc: i32,
    dr: i32,
) -> ((f64, f64), (f64, f64)) {
    let t = TILE_SIZE;
    let col = c1 as f64;
    let row = r1 as f64;
    match (dc, dr) {
        (1, 0) => (
            ((col + 1.0) * t, row * t),
            ((col + 1.0) * t, (row + 1.0) * t),
        ),
        (-1, 0) => (
            (col * t, (row + 1.0) * t),
            (col * t, row * t),
        ),
        (0, 1) => (
            ((col + 1.0) * t, (row + 1.0) * t),
            (col * t, (row + 1.0) * t),
        ),
        (0, -1) => (
            (col * t, row * t),
            ((col + 1.0) * t, row * t),
        ),
        (1, 1) => (
            ((col + 1.0) * t, (row + 1.0) * t),
            ((col + 1.0) * t, (row + 1.0) * t),
        ),
        (1, -1) => (
            ((col + 1.0) * t, row * t),
            ((col + 1.0) * t, row * t),
        ),
        (-1, 1) => (
            (col * t, (row + 1.0) * t),
            (col * t, (row + 1.0) * t),
        ),
        (-1, -1) => (
            (col * t, row * t),
            (col * t, row * t),
        ),
        _ => unreachable!(),
    }
}

/// Post-processes an A* path using the Simple Stupid Funnel Algorithm (SSFA).
/// Returns waypoints at tile vertices (not centers) forming the shortest path
/// through the corridor of cells.
pub fn funnel_algorithm(
    cells: &[(u32, u32)],
    start: (f64, f64),
    end: (f64, f64),
) -> Vec<(f64, f64)> {
    if cells.len() <= 1 {
        return vec![end];
    }
    if cells.len() == 2 {
        return vec![start, end];
    }

    let n = cells.len() - 1;
    let mut portals = Vec::with_capacity(n + 2);
    portals.push((start, start));

    for i in 0..n {
        let (c1, r1) = cells[i];
        let (c2, r2) = cells[i + 1];
        let dc = c2 as i32 - c1 as i32;
        let dr = r2 as i32 - r1 as i32;
        portals.push(portal_vertices(c1, r1, dc, dr));
    }

    portals.push((end, end));

    let mut waypoints = vec![start];
    let total = portals.len();

    let mut apex = start;
    let (mut left, mut right) = portals[1];

    let mut i = 2usize;
    while i < total {
        let (portal_left, portal_right) = portals[i];

        // Point portal (diagonal or end) — just set both sides, no cross check
        if portal_left == portal_right {
            if cross(apex, left, portal_left) >= 0.0 {
                left = portal_left;
            }
            if cross(apex, right, portal_right) <= 0.0 {
                right = portal_right;
            }
            i += 1;
            continue;
        }

        // Tighten left
        if cross(apex, left, portal_left) >= 0.0 {
            if apex != left && cross(apex, right, portal_left) > 0.0 {
                waypoints.push(right);
                apex = right;
                left = apex;
                right = apex;
                continue;
            }
            left = portal_left;
        }

        // Tighten right
        if cross(apex, right, portal_right) <= 0.0 {
            if apex != right && cross(apex, left, portal_right) < 0.0 {
                waypoints.push(left);
                apex = left;
                left = apex;
                right = apex;
                continue;
            }
            right = portal_right;
        }

        i += 1;
    }

    waypoints.push(end);
    waypoints
}

/// Validates that no segment between consecutive waypoints crosses an impassable
/// cell. When a segment does cross, walks forward through the original A* path
/// cells and inserts the first intermediate cell centre that breaks the unsafe
/// shortcut (the sub-segment from `from` to that centre must be safe).
pub fn ensure_passable_waypoints(
    waypoints: &[(f64, f64)],
    cells: &[(u32, u32)],
    grid: &GridResource,
    defs: &BiomeDefinitions,
) -> Vec<(f64, f64)> {
    let mut result: Vec<(f64, f64)> = Vec::new();
    if waypoints.is_empty() {
        return result;
    }
    result.push(waypoints[0]);
    let mut target_idx = 1usize;
    while target_idx < waypoints.len() {
        let from = *result.last().unwrap();
        let to = waypoints[target_idx];
        let line_cells = cells_on_line(from, to, grid.width, grid.height);
        let safe = line_cells.iter().all(|&(cx, cy)| {
            let idx = (cy as usize) * (grid.width as usize) + (cx as usize);
            let biome = grid.biome_ids.get(idx).copied().unwrap_or(0);
            defs.is_passable(biome)
        });
        if safe {
            result.push(to);
            target_idx += 1;
        } else {
            // Find the farthest A* cell from `from` whose centre still creates
            // a safe sub-segment.  By walking backward (from the end of the
            // cell list toward the start), we minimise the number of inserted
            // waypoints while still guaranteeing the segment is safe.
            let from_cell = pixel_to_tile(from.0, from.1);
            let start_idx = cells.iter().position(|&c| c == from_cell).unwrap_or(0);
            let mut best: Option<(f64, f64)> = None;
            for j in ((start_idx + 1)..cells.len()).rev() {
                let mid = tile_to_pixel(cells[j].0, cells[j].1);
                let sub = cells_on_line(from, mid, grid.width, grid.height);
                let sub_safe = sub.iter().all(|&(cx, cy)| {
                    let idx = (cy as usize) * (grid.width as usize) + (cx as usize);
                    let biome = grid.biome_ids.get(idx).copied().unwrap_or(0);
                    defs.is_passable(biome)
                });
                if sub_safe {
                    best = Some(mid);
                    break;
                }
            }
            if let Some(mid) = best {
                if mid == to {
                    result.push(to);
                    target_idx += 1;
                } else {
                    result.push(mid);
                }
            } else {
                // No safe intermediate found — push `to` anyway and move on
                result.push(to);
                target_idx += 1;
            }
        }
    }
    result
}

/// Post-processes centered waypoints to prevent diagonal movement that clips
/// the corner of an impassable cell. For any two consecutive waypoints whose
/// cells are diagonally adjacent (|dc|=1, |dr|=1), checks the two cells sharing
/// the crossed corner. If either is impassable, inserts the centre of the
/// passable shared-edge cell as an intermediate waypoint.
pub fn avoid_corner_clipping(
    waypoints: &[(f64, f64)],
    grid: &GridResource,
    defs: &BiomeDefinitions,
) -> Vec<(f64, f64)> {
    if waypoints.len() < 2 {
        return waypoints.to_vec();
    }
    let mut result = Vec::new();
    result.push(waypoints[0]);
    for i in 1..waypoints.len() {
        let prev = result.last().unwrap();
        let cur = waypoints[i];
        let (pc, pr) = pixel_to_tile(prev.0, prev.1);
        let (cc, cr) = pixel_to_tile(cur.0, cur.1);
        let dc = cc as i32 - pc as i32;
        let dr = cr as i32 - pr as i32;
        if dc.abs() == 1 && dr.abs() == 1 {
            // Diagonal move — check the two corner-sharing cells
            let corner_a = (pc.wrapping_add_signed(dc), pr);
            let corner_b = (pc, pr.wrapping_add_signed(dr));
            let mut need_break = false;
            for &(cx, cy) in &[corner_a, corner_b] {
                if cx < grid.width && cy < grid.height {
                    let idx = (cy as usize) * (grid.width as usize) + (cx as usize);
                    let biome = grid.biome_ids.get(idx).copied().unwrap_or(0);
                    if !defs.is_passable(biome) {
                        need_break = true;
                        break;
                    }
                }
            }
            if need_break {
                for &(cx, cy) in &[corner_a, corner_b] {
                    if cx < grid.width && cy < grid.height {
                        let idx = (cy as usize) * (grid.width as usize) + (cx as usize);
                        let biome = grid.biome_ids.get(idx).copied().unwrap_or(0);
                        if defs.is_passable(biome) {
                            result.push(tile_to_pixel(cx, cy));
                            break;
                        }
                    }
                }
            }
        }
        result.push(cur);
    }
    result
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

    // ─── portal_vertices tests ────────────────────────────────────────────────

    #[test]
    fn portal_right() {
        let (l, r) = portal_vertices(1, 2, 1, 0);
        assert_eq!(l, (64.0, 64.0));
        assert_eq!(r, (64.0, 96.0));
    }

    #[test]
    fn portal_left() {
        let (l, r) = portal_vertices(1, 2, -1, 0);
        assert_eq!(l, (32.0, 96.0));
        assert_eq!(r, (32.0, 64.0));
    }

    #[test]
    fn portal_down() {
        let (l, r) = portal_vertices(1, 2, 0, 1);
        assert_eq!(l, (64.0, 96.0));
        assert_eq!(r, (32.0, 96.0));
    }

    #[test]
    fn portal_up() {
        let (l, r) = portal_vertices(1, 2, 0, -1);
        assert_eq!(l, (32.0, 64.0));
        assert_eq!(r, (64.0, 64.0));
    }

    #[test]
    fn portal_diagonal_down_right() {
        let (l, r) = portal_vertices(1, 2, 1, 1);
        assert_eq!(l, (64.0, 96.0));
        assert_eq!(r, (64.0, 96.0));
    }

    #[test]
    fn portal_diagonal_up_right() {
        let (l, r) = portal_vertices(1, 2, 1, -1);
        assert_eq!(l, (64.0, 64.0));
        assert_eq!(r, (64.0, 64.0));
    }

    #[test]
    fn portal_diagonal_down_left() {
        let (l, r) = portal_vertices(1, 2, -1, 1);
        assert_eq!(l, (32.0, 96.0));
        assert_eq!(r, (32.0, 96.0));
    }

    #[test]
    fn portal_diagonal_up_left() {
        let (l, r) = portal_vertices(1, 2, -1, -1);
        assert_eq!(l, (32.0, 64.0));
        assert_eq!(r, (32.0, 64.0));
    }

    // ─── funnel_algorithm tests ───────────────────────────────────────────────

    fn close_enough(a: (f64, f64), b: (f64, f64)) -> bool {
        (a.0 - b.0).abs() < 0.01 && (a.1 - b.1).abs() < 0.01
    }

    #[test]
    fn funnel_straight_path_no_extra_waypoints() {
        // Straight rightward path — no extra waypoints needed
        let cells = [(0u32, 0u32), (1, 0), (2, 0), (3, 0)];
        let path = funnel_algorithm(&cells, (16.0, 16.0), (100.0, 16.0));
        assert_eq!(path.len(), 2, "should only have start and end: {:?}", path);
        assert!(close_enough(path[0], (16.0, 16.0)));
        assert!(close_enough(path[1], (100.0, 16.0)));
    }

    #[test]
    fn funnel_l_shaped_path() {
        // L-shaped path: right then down — SSFA finds direct line clear
        let cells = [(0u32, 0u32), (1, 0), (1, 1)];
        let path = funnel_algorithm(&cells, (16.0, 16.0), (48.0, 48.0));
        assert_eq!(path.len(), 2, "direct line is clear, should be start+end: {:?}", path);
        assert!(close_enough(path[0], (16.0, 16.0)));
        assert!(close_enough(path[1], (48.0, 48.0)));
    }

    #[test]
    fn funnel_s_shaped_path() {
        // S-shaped: right, down, right — direct line is clear
        let cells = [(0u32, 0u32), (1, 0), (1, 1), (2, 1)];
        let path = funnel_algorithm(&cells, (16.0, 16.0), (80.0, 48.0));
        assert_eq!(path.len(), 2, "direct line is clear, should be start+end: {:?}", path);
        assert!(close_enough(path[0], (16.0, 16.0)));
        assert!(close_enough(path[1], (80.0, 48.0)));
    }

    #[test]
    fn funnel_winding_path() {
        // Winding path around a corner in wall — forces funnel waypoints
        // Cells go up, right, down — forming a Z shape
        let cells = [(2u32, 2u32), (2, 1), (3, 1), (3, 2)];
        let path = funnel_algorithm(&cells, (80.0, 80.0), (112.0, 80.0));
        assert!(path.len() >= 2, "winding path should have at least start+end: {:?}", path);
        assert!(close_enough(path[0], (80.0, 80.0)));
        assert!(close_enough(path[path.len()-1], (112.0, 80.0)));
    }

    #[test]
    fn funnel_diagonal_path() {
        // Diagonal path — no extra waypoints (portals are points)
        let cells = [(0u32, 0u32), (1, 1), (2, 2)];
        let path = funnel_algorithm(&cells, (16.0, 16.0), (80.0, 80.0));
        assert_eq!(path.len(), 2, "diagonal should have only start and end: {:?}", path);
        assert!(close_enough(path[0], (16.0, 16.0)));
        assert!(close_enough(path[1], (80.0, 80.0)));
    }

    #[test]
    fn funnel_one_cell() {
        // Single cell — no post-processing
        let cells = [(2u32, 3u32)];
        let path = funnel_algorithm(&cells, (16.0, 16.0), (50.0, 70.0));
        assert_eq!(path, vec![(50.0, 70.0)]);
    }

    #[test]
    fn funnel_two_cells() {
        // Two cells — no post-processing
        let cells = [(0u32, 0u32), (1, 0)];
        let path = funnel_algorithm(&cells, (16.0, 16.0), (48.0, 16.0));
        assert_eq!(path, vec![(16.0, 16.0), (48.0, 16.0)]);
    }

    #[test]
    fn funnel_path_does_not_cross_mountain() {
        // Unit at (0,0), mountain at (1,0), target cell (2,0) is passable.
        // A* goes around the mountain via (0,1)→(1,1)→(2,1)→(2,0).
        // Raw funnel produces waypoints where segment (32,32)→(80,16) crosses (1,0).
        // ensure_passable_waypoints must insert a safe intermediate waypoint.
        let defs = impassable_defs();
        let grid = grid_with_walls(5, 5, &[(1, 0)]);
        let cells = find_path(&grid, &defs, (0, 0), (2, 0))
            .expect("path should exist around the mountain");
        assert!(!cells.contains(&(1, 0)), "A* path must not include the mountain cell");

        let start = tile_to_pixel(0, 0);
        let end = tile_to_pixel(2, 0);
        let waypoints = funnel_algorithm(&cells, start, end);

        // Run through the safety filter
        let safe = ensure_passable_waypoints(&waypoints, &cells, &grid, &defs);
        assert!(safe.len() >= 2, "should have at least start and end");

        // Verify no segment crosses the mountain
        for i in 1..safe.len() {
            let from = safe[i - 1];
            let to = safe[i];
            let line_cells = cells_on_line(from, to, grid.width, grid.height);
            assert!(
                !line_cells.contains(&(1, 0)),
                "safe segment {:?}→{:?} still crosses (1,0); line cells: {:?}",
                from, to, line_cells
            );
        }
    }
}
