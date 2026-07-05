// Hand-maintained TypeScript types for the WASM engine.
// `scripts/build-wasm.sh` regenerates engine.js / engine_bg.wasm with
// --no-typescript, so this .d.ts is the source of truth for types.

/** Snapshot returned by `grid_snapshot`. */
export interface GridSnapshot {
  width: number;
  height: number;
  /** Biome index per cell, length = width * height. Cell (x,y) is at y*width+x. */
  biomes: Uint8Array;
}

/** Single-cell lookup result returned by `cell_at`. */
export interface CellData {
  /** Biome index 0..=3, or 255 when the coordinate is out of bounds. */
  biome: number;
  resources: number;
}

export default function init(module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;

export function initSync(module: InitInput): InitOutput;

export function create_grid(seed: bigint, width: number, height: number): number;
export function grid_snapshot(handle: number): GridSnapshot | null;
export function cell_at(handle: number, x: number, y: number): CellData;

// The shapes below mirror wasm-bindgen's standard boot types. They're kept
// loose because the generated engine.js defines its own `InitInput`/
// `InitOutput` at runtime; callers normally just `await init()`.

type InitInput =
  | string
  | URL
  | Uint8Array
  | Request
  | Response
  | WebAssembly.Module;

type InitOutput = typeof import("./engine");

