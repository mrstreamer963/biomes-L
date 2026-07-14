import { Container, Graphics } from "pixi.js";
import type { BiomeDefinition } from "../config/biomeConfig";

export const TILE_SIZE = 32;

// Below this on-screen tile size, grid lines sit closer together than a
// screen pixel and alias into Moire stripes instead of a faint mesh — hide
// the grid rather than let it visually degrade.
const GRID_MIN_SCREEN_TILE_PX = 6;

export interface TileMap {
  container: Container;
  grid: Graphics;
  gridWidth: number;
  gridHeight: number;
}

// Redraws the tile-boundary grid so its lines are always exactly 1 *screen*
// pixel wide, regardless of zoom.
//
// Each line is a filled hairline rect rather than a stroked path. Hundreds
// of independent moveTo/lineTo segments stroked in one call go through the
// stroke tessellator's join/cap handling, which produces visibly uneven
// results across the mesh (some segments render thinner or drop out).
// Filled rects share the same simple, reliable path already used for the
// tile fills below and render evenly regardless of count.
export function updateGrid(tileMap: TileMap, scale: number) {
  const { grid, gridWidth, gridHeight } = tileMap;

  grid.visible = scale * TILE_SIZE >= GRID_MIN_SCREEN_TILE_PX;
  if (!grid.visible) return;

  const mapW = gridWidth * TILE_SIZE;
  const mapH = gridHeight * TILE_SIZE;
  const lineWidth = 1 / Math.max(scale, 0.001);
  const half = lineWidth / 2;

  grid.clear();
  for (let x = 0; x <= gridWidth; x++) {
    const px = x * TILE_SIZE;
    grid.rect(px - half, 0, lineWidth, mapH).fill({ color: 0xffffff, alpha: 0.1 });
  }
  for (let y = 0; y <= gridHeight; y++) {
    const py = y * TILE_SIZE;
    grid.rect(0, py - half, mapW, lineWidth).fill({ color: 0xffffff, alpha: 0.1 });
  }
}

export function createTileMap(
  biomeIds: Uint16Array,
  biomeDefinitions: BiomeDefinition[],
  width: number,
  height: number
): TileMap {
  const container = new Container();
  const tiles = new Graphics();

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const id = biomeIds[y * width + x];
      const def = biomeDefinitions[id];
      const color = def ? def.color : 0x000000;
      // Overlap tiles by half a pixel on each side so floating-point rounding
      // in the WebGL transform never leaves a sub-pixel gap between neighbors
      // (visible as background-colored seams once the camera zooms in).
      tiles
        .rect(x * TILE_SIZE - 0.5, y * TILE_SIZE - 0.5, TILE_SIZE + 1, TILE_SIZE + 1)
        .fill({ color });
    }
  }

  container.addChild(tiles);

  // Line geometry is filled in by updateGrid() once the initial zoom scale
  // is known, so the stroke width can be computed correctly from the start.
  const grid = new Graphics();
  container.addChild(grid);

  return { container, grid, gridWidth: width, gridHeight: height };
}
