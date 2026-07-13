import { Container, Graphics } from "pixi.js";
import type { BiomeDefinition } from "../config/biomeConfig";
import type { UnitData } from "../wasm/engine";
import { TILE_SIZE } from "./MapRenderer";

export const ZONE_FACTOR = 5;
export const MARKER_SCREEN_PX = 14;

const BACKGROUND_COLOR = 0x0a0a12;
const GRID_LINE_COLOR = 0x4488ff;
const GRID_LINE_ALPHA = 0.15;

export function downsampleBiomes(
  biomeIds: Uint16Array,
  width: number,
  height: number,
  factor: number
): { data: Uint16Array; zonesX: number; zonesY: number } {
  const zonesX = Math.ceil(width / factor);
  const zonesY = Math.ceil(height / factor);
  const data = new Uint16Array(zonesX * zonesY);

  for (let zy = 0; zy < zonesY; zy++) {
    for (let zx = 0; zx < zonesX; zx++) {
      const counts = new Map<number, number>();
      const x0 = zx * factor;
      const y0 = zy * factor;
      const x1 = Math.min(x0 + factor, width);
      const y1 = Math.min(y0 + factor, height);

      for (let y = y0; y < y1; y++) {
        for (let x = x0; x < x1; x++) {
          const id = biomeIds[y * width + x];
          counts.set(id, (counts.get(id) ?? 0) + 1);
        }
      }

      let bestId = 0;
      let bestCount = -1;
      for (const [id, count] of counts) {
        if (count > bestCount) {
          bestCount = count;
          bestId = id;
        }
      }
      data[zy * zonesX + zx] = bestId;
    }
  }

  return { data, zonesX, zonesY };
}

export function tacticalColorFor(color: number): number {
  const r = (color >> 16) & 0xff;
  const g = (color >> 8) & 0xff;
  const b = color & 0xff;

  const darken = 0.45;
  const gray = (r + g + b) / 3;
  const desaturate = 0.5;

  const mix = (channel: number) => {
    const darkened = channel * darken;
    return darkened * (1 - desaturate) + gray * darken * desaturate;
  };

  const nr = Math.round(mix(r));
  const ng = Math.round(mix(g));
  const nb = Math.round(mix(b));

  return (nr << 16) | (ng << 8) | nb;
}

export class TacticalRenderer {
  readonly container = new Container();
  private zoneLayer = new Graphics();
  private markerLayer = new Graphics();

  private units: UnitData[] = [];
  private selectedUnitId: number | null = null;
  private viewScale = 1;

  constructor() {
    this.container.addChild(this.zoneLayer);
    this.container.addChild(this.markerLayer);
  }

  buildMap(
    biomeIds: Uint16Array,
    defs: BiomeDefinition[],
    width: number,
    height: number
  ): void {
    const g = this.zoneLayer;
    g.clear();

    const mapW = width * TILE_SIZE;
    const mapH = height * TILE_SIZE;
    g.rect(0, 0, mapW, mapH).fill({ color: BACKGROUND_COLOR });

    const { data, zonesX, zonesY } = downsampleBiomes(biomeIds, width, height, ZONE_FACTOR);
    const zoneSize = ZONE_FACTOR * TILE_SIZE;

    for (let zy = 0; zy < zonesY; zy++) {
      for (let zx = 0; zx < zonesX; zx++) {
        const id = data[zy * zonesX + zx];
        const def = defs[id];
        const color = def ? tacticalColorFor(def.color) : BACKGROUND_COLOR;
        const w = Math.min(zoneSize, mapW - zx * zoneSize);
        const h = Math.min(zoneSize, mapH - zy * zoneSize);
        g.rect(zx * zoneSize, zy * zoneSize, w, h).fill({ color });
      }
    }

    for (let zx = 0; zx <= zonesX; zx++) {
      const px = Math.min(zx * zoneSize, mapW);
      g.moveTo(px, 0).lineTo(px, mapH);
    }
    for (let zy = 0; zy <= zonesY; zy++) {
      const py = Math.min(zy * zoneSize, mapH);
      g.moveTo(0, py).lineTo(mapW, py);
    }
    g.stroke({ width: 1, color: GRID_LINE_COLOR, alpha: GRID_LINE_ALPHA });
  }

  updateUnits(units: UnitData[], selectedUnitId: number | null): void {
    this.units = units;
    this.selectedUnitId = selectedUnitId;
    this.drawMarkers();
  }

  setViewScale(scale: number): void {
    this.viewScale = scale;
    this.drawMarkers();
  }

  private drawMarkers(): void {
    const g = this.markerLayer;
    g.clear();

    const radius = MARKER_SCREEN_PX / Math.max(this.viewScale, 0.001);

    for (const unit of this.units) {
      const isSelected = unit.id === this.selectedUnitId;
      const bodyColor = unit.team === 0 ? 0xaa44cc : 0xcc4444;

      if (isSelected) {
        this.drawDiamond(g, unit.x, unit.y, radius + radius * 0.35, 0xffff00, 0.6);
      }

      this.drawDiamond(g, unit.x, unit.y, radius, bodyColor, 0.95);
      this.strokeDiamond(g, unit.x, unit.y, radius);
    }
  }

  private drawDiamond(g: Graphics, x: number, y: number, r: number, color: number, alpha: number): void {
    g.poly([x, y - r, x + r, y, x, y + r, x - r, y]).fill({ color, alpha });
  }

  private strokeDiamond(g: Graphics, x: number, y: number, r: number): void {
    g.poly([x, y - r, x + r, y, x, y + r, x - r, y]).stroke({ width: 1.5, color: 0x000000, alpha: 0.5 });
  }

  destroy(): void {
    this.zoneLayer.destroy();
    this.markerLayer.destroy();
    this.container.destroy({ children: true });
  }
}
