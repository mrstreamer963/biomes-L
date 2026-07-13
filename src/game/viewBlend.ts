export type ViewMode = "detailed" | "tactical";

export interface ViewBlendConfig {
  detailedFullAt: number;
  tacticalFullAt: number;
  tacticalEnterAt: number;
  tacticalExitAt: number;
}

export const DEFAULT_VIEW_BLEND: ViewBlendConfig = {
  detailedFullAt: 0.45,
  tacticalFullAt: 0.28,
  tacticalEnterAt: 0.3,
  tacticalExitAt: 0.42,
};

function smoothstep(edge0: number, edge1: number, x: number): number {
  const t = Math.min(1, Math.max(0, (x - edge0) / (edge1 - edge0)));
  return t * t * (3 - 2 * t);
}

export function tacticalBlendFor(scale: number, cfg: ViewBlendConfig = DEFAULT_VIEW_BLEND): number {
  // scale >= detailedFullAt -> 0 (fully detailed)
  // scale <= tacticalFullAt -> 1 (fully tactical)
  return smoothstep(cfg.detailedFullAt, cfg.tacticalFullAt, scale);
}

export function resolveViewMode(
  scale: number,
  prev: ViewMode,
  cfg: ViewBlendConfig = DEFAULT_VIEW_BLEND
): ViewMode {
  if (prev === "detailed" && scale < cfg.tacticalEnterAt) return "tactical";
  if (prev === "tactical" && scale > cfg.tacticalExitAt) return "detailed";
  return prev;
}

export function approach(current: number, target: number, dtMs: number, tauMs = 120): number {
  if (dtMs <= 0) return current;
  const factor = 1 - Math.exp(-dtMs / tauMs);
  return current + (target - current) * factor;
}
