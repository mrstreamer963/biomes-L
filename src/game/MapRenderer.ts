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

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const id = biomeIds[y * width + x];
      const def = biomeDefinitions[id];
      const color = def ? def.color : 0x000000;

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
