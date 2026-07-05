import { Container, Graphics } from "pixi.js";
import type { UnitData } from "../wasm/engine";

export interface HitTestResult {
  unitId: number;
}

export class UnitManager {
  public readonly container = new Container();
  private graphics = new Graphics();
  private units: Map<number, UnitData> = new Map();
  private selectedUnitId: number | null = null;

  constructor() {
    this.container.addChild(this.graphics);
  }

  update(unitData: UnitData[], selectedUnitId: number | null): void {
    this.selectedUnitId = selectedUnitId;
    this.graphics.clear();

    // Update the unit map for hit testing
    this.units.clear();
    for (const unit of unitData) {
      this.units.set(unit.id, unit);
      this.drawUnit(unit);
    }
  }

  private drawUnit(unit: UnitData): void {
    const isSelected = unit.id === this.selectedUnitId;
    const radius = unit.unit_type === "Soldier" ? 13 : 10;
    const g = this.graphics;

    // Unit body color based on team
    const bodyColor = unit.team === 0 ? 0xaa44cc : 0xcc4444;
    const alpha = 0.9;

    // Selection ring
    if (isSelected) {
      g.circle(unit.x, unit.y, radius + 3)
        .fill({ color: 0xffff00, alpha: 0.6 });
    }

    // Unit body
    g.circle(unit.x, unit.y, radius)
      .fill({ color: bodyColor, alpha });

    // Unit border
    g.circle(unit.x, unit.y, radius)
      .stroke({ width: 1.5, color: 0x000000, alpha: 0.5 });

    // Health bar background
    const barWidth = radius * 2 + 4;
    const barHeight = 3;
    const barX = unit.x - barWidth / 2;
    const barY = unit.y - radius - 7;

    g.rect(barX, barY, barWidth, barHeight)
      .fill({ color: 0x333333 });

    // Health bar fill
    const healthRatio = unit.max_health > 0 ? unit.health / unit.max_health : 0;
    const healthColor = healthRatio > 0.5 ? 0x44cc44 : healthRatio > 0.25 ? 0xcccc44 : 0xcc4444;

    if (healthRatio > 0) {
      g.rect(barX, barY, barWidth * healthRatio, barHeight)
        .fill({ color: healthColor });
    }
  }

  hitTest(worldX: number, worldY: number): HitTestResult | null {
    // Iterate in reverse order (top unit first)
    const entries = Array.from(this.units.entries()).reverse();

    for (const [id, unit] of entries) {
      const dx = worldX - unit.x;
      const dy = worldY - unit.y;
      const radius = unit.unit_type === "Soldier" ? 13 : 10;
      const dist = Math.sqrt(dx * dx + dy * dy);

      if (dist <= radius + 2) {
        return { unitId: id };
      }
    }

    return null;
  }
}