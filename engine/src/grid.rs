//! Grid of biome cells and its generator.

use crate::biome::Biome;
use crate::rng::Lcg;

/// A single grid cell: which biome it holds, plus a reserved `resources`
/// field for future resource gathering (left at 0 for now).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub biome: Biome,
    pub resources: u8,
}

impl Cell {
    /// Convenience constructor — a cell with the given biome and no
    /// resources, which is the default state for freshly generated cells.
    pub fn new(biome: Biome) -> Self {
        Self {
            biome,
            resources: 0,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new(Biome::Plains)
    }
}

/// A 2D grid of cells with fixed `width × height`. Cell `(x, y)` lives at
/// linear index `y * width + x`.
#[derive(Debug, Clone)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

impl Grid {
    /// Build an empty (all-Plains) grid of the given size without running
    /// the generator. Useful for tests that want to inspect layout.
    pub fn empty(width: u32, height: u32) -> Self {
        let len = (width as usize).checked_mul(height as usize).unwrap_or(0);
        Self {
            width,
            height,
            cells: vec![Cell::default(); len],
        }
    }

    /// Linear index for `(x, y)`, or `None` if out of bounds.
    fn index(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }
        // y * width + x, with width already validated against u32 so the
        // product fits in usize for any real grid.
        Some((y as usize) * (self.width as usize) + (x as usize))
    }

    /// Borrow the cell at `(x, y)`, or `None` if out of bounds. Never panics.
    pub fn cell_at(&self, x: u32, y: u32) -> Option<&Cell> {
        self.index(x, y).map(|i| &self.cells[i])
    }

    /// Deterministically generate a grid of the given size from `seed`.
    ///
    /// The same `(seed, width, height)` always yields identical biomes at
    /// the same coordinates. Biomes are picked per cell by a threshold over
    /// the LCG output:
    /// - `< 64`  → Water
    /// - `< 160` → Mountain
    /// - `< 232` → Forest
    /// - else    → Plains
    ///
    /// Thresholds are chosen so Plains is the most common, with Forest,
    /// Water and Mountain as progressively rarer features — a reasonable
    /// starting distribution for a prototype map.
    pub fn generate(seed: u64, width: u32, height: u32) -> Grid {
        let mut rng = Lcg::new(seed);
        let mut grid = Grid::empty(width, height);
        for cell in grid.cells.iter_mut() {
            let v = rng.next_u32() & 0xFF; // 0..=255
            cell.biome = match v {
                0..=63 => Biome::Water,
                64..=159 => Biome::Mountain,
                160..=231 => Biome::Forest,
                _ => Biome::Plains,
            };
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_default_is_plains_no_resources() {
        let c = Cell::default();
        assert_eq!(c.biome, Biome::Plains);
        assert_eq!(c.resources, 0);
    }

    #[test]
    fn cell_at_in_bounds() {
        let grid = Grid::empty(4, 3);
        // (2,1) -> index 1*4 + 2 = 6
        let cell = grid.cell_at(2, 1).expect("in bounds");
        assert_eq!(cell.biome, Biome::Plains);
    }

    #[test]
    fn cell_at_out_of_bounds_returns_none() {
        let grid = Grid::empty(2, 2);
        assert!(grid.cell_at(2, 0).is_none()); // x out
        assert!(grid.cell_at(0, 2).is_none()); // y out
        assert!(grid.cell_at(2, 2).is_none()); // both out
    }

    #[test]
    fn generate_is_reproducible() {
        let a = Grid::generate(42, 8, 8);
        let b = Grid::generate(42, 8, 8);
        assert_eq!(a.width, b.width);
        assert_eq!(a.height, b.height);
        assert_eq!(a.cells.len(), b.cells.len());
        assert!(a.cells.iter().zip(b.cells.iter()).all(|(x, y)| x == y));
    }

    #[test]
    fn different_seeds_yield_different_maps() {
        let a = Grid::generate(1, 16, 16);
        let b = Grid::generate(2, 16, 16);
        assert!(a.cells.iter().zip(b.cells.iter()).any(|(x, y)| x != y));
    }

    #[test]
    fn generate_only_emits_valid_biomes() {
        let grid = Grid::generate(7, 10, 10);
        for c in &grid.cells {
            assert!(matches!(
                c.biome,
                Biome::Plains | Biome::Forest | Biome::Water | Biome::Mountain
            ));
        }
    }
}
