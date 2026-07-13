import { Container, Graphics } from "pixi.js";
import type { UnitData } from "../wasm/engine";

export interface HitTestResult {
  unitId: number;
}

export class UnitManager {
  public readonly container = new Container();
  private unitLayer = new Graphics();
  private debugLayer = new Graphics();
  private units: Map<number, UnitData> = new Map();
  private selectedUnitId: number | null = null;

  constructor() {
    this.container.addChild(this.debugLayer);
    this.container.addChild(this.unitLayer);
  }

  update(unitData: UnitData[], selectedUnitId: number | null): void {
    this.selectedUnitId = selectedUnitId;
    this.unitLayer.clear();
    this.debugLayer.clear();

    this.units.clear();
    for (const unit of unitData) {
      this.units.set(unit.id, unit);
      if (unit.debug) {
        this.drawDebug(unit);
      }
      this.drawUnit(unit);
    }
  }

  private drawDebug(unit: UnitData): void {
    const g = this.debugLayer;
    const wp = unit.path;

    if (wp.length === 0) return;

    // Draw waypoint path as connected line segments
    const points: [number, number][] = [[unit.x, unit.y]];
    for (const p of wp) {
      points.push(p);
    }

    for (let i = 0; i < points.length - 1; i++) {
      const [x1, y1] = points[i];
      const [x2, y2] = points[i + 1];
      g.moveTo(x1, y1)
        .lineTo(x2, y2)
        .stroke({ width: 1.5, color: 0x00ffff, alpha: 0.7 });
    }

    // Draw waypoint dots
    for (const [wx, wy] of wp) {
      g.circle(wx, wy, 3)
        .fill({ color: 0x00ffff, alpha: 0.8 });
    }

    // Draw target crosshair
    if (unit.target) {
      const [tx, ty] = unit.target;
      const s = 8;
      g.moveTo(tx - s, ty)
        .lineTo(tx + s, ty)
        .stroke({ width: 2, color: 0xff4444, alpha: 0.9 });
      g.moveTo(tx, ty - s)
        .lineTo(tx, ty + s)
        .stroke({ width: 2, color: 0xff4444, alpha: 0.9 });
      g.circle(tx, ty, s)
        .stroke({ width: 1.5, color: 0xff4444, alpha: 0.6 });
    }
  }

  private drawUnit(unit: UnitData): void {
    const isSelected = unit.id === this.selectedUnitId;
    const radius = unit.unit_type === "Soldier" ? 13 : 10;
    const g = this.unitLayer;

    const bodyColor = unit.team === 0 ? 0xaa44cc : 0xcc4444;
    const alpha = 0.9;

    if (isSelected) {
      g.circle(unit.x, unit.y, radius + 3)
        .fill({ color: 0xffff00, alpha: 0.6 });
    }

    g.circle(unit.x, unit.y, radius)
      .fill({ color: bodyColor, alpha });

    g.circle(unit.x, unit.y, radius)
      .stroke({ width: 1.5, color: 0x000000, alpha: 0.5 });

    const barWidth = radius * 2 + 4;
    const barHeight = 3;
    const barX = unit.x - barWidth / 2;
    const barY = unit.y - radius - 7;

    g.rect(barX, barY, barWidth, barHeight)
      .fill({ color: 0x333333 });

    const healthRatio = unit.max_health > 0 ? unit.health / unit.max_health : 0;
    const healthColor = healthRatio > 0.5 ? 0x44cc44 : healthRatio > 0.25 ? 0xcccc44 : 0xcc4444;

    if (healthRatio > 0) {
      g.rect(barX, barY, barWidth * healthRatio, barHeight)
        .fill({ color: healthColor });
    }
  }

  hitTest(worldX: number, worldY: number, minRadius = 0): HitTestResult | null {
    const entries = Array.from(this.units.entries()).reverse();

    for (const [id, unit] of entries) {
      const dx = worldX - unit.x;
      const dy = worldY - unit.y;
      const radius = unit.unit_type === "Soldier" ? 13 : 10;
      const dist = Math.sqrt(dx * dx + dy * dy);

      if (dist <= Math.max(radius + 2, minRadius)) {
        return { unitId: id };
      }
    }

    return null;
  }
}