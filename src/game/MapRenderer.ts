import { Container, Graphics } from "pixi.js";
import type { BiomeDefinition } from "../config/biomeConfig";

export const TILE_SIZE = 32;

export function createTileMap(
  biomeIds: Uint16Array,
  biomeDefinitions: BiomeDefinition[],
  width: number,
  height: number
): Container {
  const container = new Container();
  const tiles = new Graphics();

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const id = biomeIds[y * width + x];
      const def = biomeDefinitions[id];
      const color = def ? def.color : 0x000000;
      tiles.rect(x * TILE_SIZE, y * TILE_SIZE, TILE_SIZE, TILE_SIZE).fill({ color });
    }
  }

  container.addChild(tiles);

  const grid = new Graphics();
  const mapW = width * TILE_SIZE;
  const mapH = height * TILE_SIZE;

  for (let x = 0; x <= width; x++) {
    const px = x * TILE_SIZE;
    grid.moveTo(px, 0).lineTo(px, mapH);
  }
  for (let y = 0; y <= height; y++) {
    const py = y * TILE_SIZE;
    grid.moveTo(0, py).lineTo(mapW, py);
  }
  grid.stroke({ width: 1, color: 0xffffff, alpha: 0.1 });

  container.addChild(grid);

  return container;
}
