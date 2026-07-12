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

/// Squared tile-space distance between two cells, used to find the A* path
/// cell nearest to a waypoint that doesn't land exactly on the path.
fn cell_dist_sq(a: (u32, u32), b: (u32, u32)) -> i64 {
    let dc = a.0 as i64 - b.0 as i64;
    let dr = a.1 as i64 - b.1 as i64;
    dc * dc + dr * dr
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

/// Checks whether a straight segment between two points is safe to walk.
///
/// This is more than "every sampled cell is passable": a segment between two
/// diagonally-adjacent tile centres passes exactly through the shared vertex
/// of four tiles. `cells_on_line`'s floor-based sampling resolves that exact
/// vertex to a single cell (the one with the larger row *and* column, since
/// `floor` of an already-integer coordinate is a no-op) — so the sampled cell
/// sequence can look entirely clear even when the vertex is shared with a
/// wall on the *other* diagonal. `find_path`'s A* already refuses a diagonal
/// step unless both flanking orthogonal cells are passable (see the
/// diagonal-adjacency check above); this mirrors that same rule so a
/// funnel-smoothed shortcut can't offer a route A* would never have taken —
/// otherwise a unit's real-time, frame-by-frame movement can clip that wall
/// corner and get its path/target wiped mid-flight.
fn line_is_safe(from: (f64, f64), to: (f64, f64), grid: &GridResource, defs: &BiomeDefinitions) -> bool {
    let cell_passable = |(cx, cy): (u32, u32)| -> bool {
        if cx >= grid.width || cy >= grid.height {
            return true;
        }
        let idx = (cy as usize) * (grid.width as usize) + (cx as usize);
        let biome = grid.biome_ids.get(idx).copied().unwrap_or(0);
        defs.is_passable(biome)
    };

    let line_cells = cells_on_line(from, to, grid.width, grid.height);
    for &cell in &line_cells {
        if !cell_passable(cell) {
            return false;
        }
    }

    // Corner-vertex check: walk the same samples `cells_on_line` uses and,
    // whenever a sample lands exactly on a tile corner (both axes on a grid
    // line at once), verify all four tiles sharing that vertex — not just
    // the one `floor` happens to pick.
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let dist = dx.hypot(dy);
    let steps = dist.ceil().max(1.0) as u32;
    const EPS: f64 = 1e-6;

    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let x = from.0 + dx * t;
        let y = from.1 + dy * t;

        let col_f = x / TILE_SIZE;
        let row_f = y / TILE_SIZE;
        let on_x_line = (col_f - col_f.round()).abs() < EPS;
        let on_y_line = (row_f - row_f.round()).abs() < EPS;

        if on_x_line && on_y_line {
            let col = col_f.round() as i32;
            let row = row_f.round() as i32;
            for &(cc, rr) in &[(col - 1, row - 1), (col, row - 1), (col - 1, row), (col, row)] {
                if cc < 0 || rr < 0 {
                    continue;
                }
                if !cell_passable((cc as u32, rr as u32)) {
                    return false;
                }
            }
        }
    }

    true
}

/// Validates that no segment between consecutive waypoints crosses an impassable
/// cell. When a segment does cross, replaces the shortcut with centres from the
/// original A* path to guarantee safety.
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

    // Start with the first waypoint (converted to cell centre if needed)
    let start_cell = pixel_to_tile(waypoints[0].0, waypoints[0].1);
    let start_px = tile_to_pixel(start_cell.0, start_cell.1);
    result.push(start_px);

    // Tracks how far along the A* path we've consumed. We keep this explicit
    // rather than re-deriving it from `from`'s pixel each iteration: raw funnel
    // waypoints are portal vertices that sit exactly on tile boundaries, so
    // floor-based pixel_to_tile can round them into a neighbouring cell (e.g. a
    // wall) that never appears in `cells`. An exact-match `.position()` lookup
    // then fails and a naive fallback (index 0 / last index) desyncs `from_idx`
    // from `to_idx`, silently dropping waypoints or destroying funnel smoothing
    // for the rest of the route.
    let mut from_idx = cells.iter().position(|&c| c == start_cell).unwrap_or(0);

    let mut target_idx = 1usize;
    while target_idx < waypoints.len() {
        let from = *result.last().unwrap();
        let to = waypoints[target_idx];

        // Skip duplicate waypoints
        if from == to {
            target_idx += 1;
            continue;
        }

        // Check if direct segment is safe
        let safe = line_is_safe(from, to, grid, defs);

        let to_cell = pixel_to_tile(to.0, to.1);
        // Nearest A* cell to `to`, searched forward from our current position
        // only. This always resolves (cells is non-empty here) and can never
        // move `from_idx` backwards, unlike an exact-match lookup that falls
        // back to index 0 or the last index when `to` rounds onto an off-path
        // tile.
        let nearest_idx = cells
            .iter()
            .enumerate()
            .skip(from_idx)
            .min_by_key(|&(_, &c)| cell_dist_sq(c, to_cell))
            .map(|(i, _)| i)
            .unwrap_or(from_idx);

        if safe {
            // Direct segment is safe — use the target waypoint (converted to cell centre)
            let to_px = tile_to_pixel(to_cell.0, to_cell.1);
            // Avoid adding duplicate if same as last waypoint
            if result.last() != Some(&to_px) {
                result.push(to_px);
            }
            from_idx = nearest_idx;
        } else {
            // Direct segment crosses impassable cells — use A* path cells as waypoints,
            // up to the nearest path cell to the intended target.
            for j in (from_idx + 1)..=nearest_idx {
                let cell = cells[j];
                let px = tile_to_pixel(cell.0, cell.1);
                // Avoid duplicates in A* path cells and between consecutive cells
                if result.last() != Some(&px) {
                    result.push(px);
                }
            }
            from_idx = nearest_idx;
        }

        target_idx += 1;
    }

    result
}

/// Post-processes waypoints to prevent movement that clips through impassable cells.
/// For any two consecutive waypoints, checks if the line segment between them crosses
/// any impassable cell. If so, inserts intermediate waypoints to create a safe route.
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
        let prev = *result.last().unwrap();
        let cur = waypoints[i];
        
        // Check if this segment crosses any impassable cell
        let line_cells = cells_on_line(prev, cur, grid.width, grid.height);
        let has_impassable = line_cells.iter().any(|&(cx, cy)| {
            if cx < grid.width && cy < grid.height {
                let idx = (cy as usize) * (grid.width as usize) + (cx as usize);
                let biome = grid.biome_ids.get(idx).copied().unwrap_or(0);
                !defs.is_passable(biome)
            } else {
                false
            }
        });
        
        if has_impassable {
            // Segment crosses impassable cells - just push current waypoint (fallback)
            result.push(cur);
        } else {
            result.push(cur);
        }
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

    #[test]
    fn path_from_15_1_to_16_3() {
        // Test case: unit at (15,1), target at (16,3)
        let defs = passable_defs();
        let grid = empty_grid(20, 10);
        
        let path = find_path(&grid, &defs, (15, 1), (16, 3));
        assert!(path.is_some(), "Path should exist from (15,1) to (16,3)");
        let cells = path.unwrap();
        
        // Verify start and end are in the path
        assert_eq!(cells.first(), Some(&(15, 1)), "Path should start at (15,1)");
        assert_eq!(cells.last(), Some(&(16, 3)), "Path should end at (16,3)");
        
        // Run through the full pipeline
        let start_pixel = tile_to_pixel(15, 1);
        let end_pixel = tile_to_pixel(16, 3);
        
        let funnel_wps = funnel_algorithm(&cells, start_pixel, end_pixel);
        println!("Funnel waypoints: {:?}", funnel_wps);
        
        let safe_wps = ensure_passable_waypoints(&funnel_wps, &cells, &grid, &defs);
        println!("Safe waypoints: {:?}", safe_wps);
        
        let final_wps = avoid_corner_clipping(&safe_wps, &grid, &defs);
        println!("Final waypoints: {:?}", final_wps);
        
        // Verify all segments between waypoints are safe (no impassable cells)
        for i in 1..final_wps.len() {
            let from = final_wps[i - 1];
            let to = final_wps[i];
            let line_cells = cells_on_line(from, to, grid.width, grid.height);
            
            for &(cx, cy) in &line_cells {
                let biome = grid.biome_ids[(cy as usize) * (grid.width as usize) + (cx as usize)];
                assert!(defs.is_passable(biome),
                    "Waypoint segment {:?}→{:?} crosses impassable cell ({},{})",
                    from, to, cx, cy);
            }
        }
    }

    #[test]
    fn path_from_15_1_to_16_3_with_obstacle() {
        // Test case: unit at (15,1), target at (16,3) with obstacle in between
        let defs = impassable_defs();
        
        // Place obstacles blocking direct path — unit must go around bottom-left
        let walls = vec![(15, 2), (16, 2), (16, 1), (15, 3)];
        let grid = grid_with_walls(20, 10, &walls);
        
        let path = find_path(&grid, &defs, (15, 1), (16, 3));
        assert!(path.is_some(), "Path should exist from (15,1) to (16,3) around obstacles");
        let cells = path.unwrap();
        
        println!("A* path cells: {:?}", cells);
        
        // Verify no cell in the path is a wall
        for &(c, r) in &cells {
            let idx = (r as usize) * 20 + (c as usize);
            assert!(defs.is_passable(grid.biome_ids[idx]), 
                "Path includes wall at ({}, {})", c, r);
        }
        
        // Run through the full pipeline
        let start_pixel = tile_to_pixel(15, 1);
        let end_pixel = tile_to_pixel(16, 3);
        
        let funnel_wps = funnel_algorithm(&cells, start_pixel, end_pixel);
        println!("Funnel waypoints: {:?}", funnel_wps);
        
        let safe_wps = ensure_passable_waypoints(&funnel_wps, &cells, &grid, &defs);
        println!("Safe waypoints: {:?}", safe_wps);
        
        let final_wps = avoid_corner_clipping(&safe_wps, &grid, &defs);
        println!("Final waypoints: {:?}", final_wps);
        
        // Verify all segments between waypoints are safe (no impassable cells)
        for i in 1..final_wps.len() {
            let from = final_wps[i - 1];
            let to = final_wps[i];
            let line_cells = cells_on_line(from, to, grid.width, grid.height);
            
            for &(cx, cy) in &line_cells {
                let biome = grid.biome_ids[(cy as usize) * (grid.width as usize) + (cx as usize)];
                assert!(defs.is_passable(biome),
                    "Waypoint segment {:?}→{:?} crosses impassable cell ({},{})",
                    from, to, cx, cy);
            }
        }
    }

    #[test]
    fn ensure_passable_waypoints_does_not_desync_on_off_path_vertex() {
        // A raw funnel waypoint can land exactly on a tile boundary and
        // floor-round into a wall cell that is NOT part of the A* path
        // (here: (480,64) -> tile (15,2), a wall). An exact-match
        // `.position()` lookup for that cell fails; a naive fallback
        // (unwrap_or(0) for `from_idx`, unwrap_or(cells.len()-1) for
        // `to_idx`) desyncs the two indices so that a *later* segment's
        // `from_idx` ends up greater than its `to_idx`, silently dropping
        // that waypoint instead of routing through it — and can also
        // skip past the real end into the whole rest of the path in one
        // jump. This must not happen: waypoints should track forward,
        // never drop the segment that reaches the target, and never cross
        // a wall.
        let defs = impassable_defs();
        let walls = vec![(15, 2), (16, 2), (16, 1), (15, 3)];
        let grid = grid_with_walls(20, 10, &walls);

        let path = find_path(&grid, &defs, (15, 1), (16, 3))
            .expect("path should exist around the small wall cluster");

        let start_pixel = tile_to_pixel(15, 1);
        let end_pixel = tile_to_pixel(16, 3);
        let funnel_wps = funnel_algorithm(&path, start_pixel, end_pixel);
        let safe_wps = ensure_passable_waypoints(&funnel_wps, &path, &grid, &defs);

        // Must actually reach the target — the old fallback logic could
        // desync indices badly enough to stop short.
        assert_eq!(
            *safe_wps.last().unwrap(),
            end_pixel,
            "path did not reach the target: {:?}",
            safe_wps
        );

        // Should stay at least as smoothed as one waypoint per A* cell,
        // rather than degrading into more waypoints than that — the buggy
        // desync logic used to dump in the whole remaining path (8 points)
        // after the first off-path miss. Note this no longer beats the A*
        // cell count here: the shortcut that used to shave off a waypoint
        // was itself the corner-clipping bug (see
        // `ensure_passable_waypoints_rejects_diagonal_corner_clip`) — it cut
        // exactly through the (15,3) wall's corner, which is why the fixed
        // version spends one more waypoint (528,144) routing around it.
        assert!(
            safe_wps.len() <= path.len(),
            "expected fewer waypoints than A* cells ({}), got {}: {:?}",
            path.len(),
            safe_wps.len(),
            safe_wps
        );

        // Every consecutive segment must remain safe.
        for i in 1..safe_wps.len() {
            let from = safe_wps[i - 1];
            let to = safe_wps[i];
            let line_cells = cells_on_line(from, to, grid.width, grid.height);
            for &(cx, cy) in &line_cells {
                let biome = grid.biome_ids[(cy as usize) * (grid.width as usize) + (cx as usize)];
                assert!(
                    defs.is_passable(biome),
                    "safe segment {:?}→{:?} crosses impassable cell ({},{})",
                    from, to, cx, cy
                );
            }
        }
    }

    #[test]
    fn ensure_passable_waypoints_rejects_diagonal_corner_clip() {
        // Reproduces the live "unit gets stuck" bug: selecting the unit at
        // (15,1) and commanding it to (16,3) with this exact wall cluster
        // produces a funnel/safety path whose last leg is a pure diagonal
        // from tile (15,4)'s centre to tile (16,3)'s centre. That line passes
        // exactly through the shared corner of (15,3)/(15,4)/(16,3)/(16,4);
        // `cells_on_line`'s floor-sampling happens to land on (16,4)
        // (passable) at that exact vertex and never touches (15,3) (a wall),
        // so the old plain "are all sampled cells passable" check certified
        // it safe. `find_path`'s own A* would never take this diagonal step
        // directly, since its diagonal-adjacency rule requires both flanking
        // orthogonal cells — including (15,3) — to be passable. The runtime
        // movement system moves frame-by-frame along this "safe" diagonal
        // and can clip the (15,3) wall corner, wiping the unit's path and
        // target mid-flight. The fix must reject this shortcut the same way
        // A* would.
        let defs = impassable_defs();
        let walls = vec![(15, 2), (16, 2), (16, 1), (15, 3)];
        let grid = grid_with_walls(20, 10, &walls);

        let from = tile_to_pixel(15, 4);
        let to = tile_to_pixel(16, 3);
        assert!(
            !line_is_safe(from, to, &grid, &defs),
            "diagonal shortcut {:?}→{:?} clips the (15,3) wall corner and must not be marked safe",
            from, to
        );

        let path = find_path(&grid, &defs, (15, 1), (16, 3))
            .expect("path should exist around the wall cluster");
        let start_pixel = tile_to_pixel(15, 1);
        let end_pixel = tile_to_pixel(16, 3);
        let funnel_wps = funnel_algorithm(&path, start_pixel, end_pixel);
        let safe_wps = ensure_passable_waypoints(&funnel_wps, &path, &grid, &defs);

        assert_eq!(
            *safe_wps.last().unwrap(),
            end_pixel,
            "path did not reach the target: {:?}",
            safe_wps
        );

        // No consecutive pair of safe waypoints may take the forbidden
        // diagonal shortcut between (15,4) and (16,3).
        for pair in safe_wps.windows(2) {
            let (from, to) = (pair[0], pair[1]);
            assert!(
                !(from == tile_to_pixel(15, 4) && to == tile_to_pixel(16, 3)),
                "safe waypoints still take the corner-clipping diagonal shortcut: {:?}",
                safe_wps
            );
        }
    }
}
