import { Container, Graphics } from "pixi.js";

export const TILE_SIZE = 32;

export const BIOME_COLORS: Record<number, number> = {
  0: 0x7ec850, // Plains
  1: 0x2d5a27, // Forest
  2: 0x3b82f6, // Water
  3: 0x8b7355, // Mountain
};

export function createTileMap(
  biomes: Uint8Array,
  width: number,
  height: number
): Container {
  const container = new Container();

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const biome = biomes[y * width + x];
      const color = BIOME_COLORS[biome] ?? 0x000000;

      const tile = new Graphics();
      tile.rect(0, 0, TILE_SIZE, TILE_SIZE).fill({ color });
      tile.rect(0, 0, TILE_SIZE, TILE_SIZE).stroke({
        width: 1,
        color: 0xffffff,
        alpha: 0.1,
      });
      tile.position.set(x * TILE_SIZE, y * TILE_SIZE);
      container.addChild(tile);
    }
  }

  return container;
}
