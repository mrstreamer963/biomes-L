export interface BiomeDefinition {
  name: string;
  passable: boolean;
  speed: number;
  color: number;
}

export interface GridSnapshot {
  width: number;
  height: number;
  biomeIds: Uint16Array;
  resources: Uint8Array;
}

export interface CellData {
  biome: number;
  resources: number;
}

export interface GenerationParams {
  seed: bigint;
  scale?: number;
  octaves?: number;
  persistence?: number;
  lacunarity?: number;
  elevationLow?: number;
  elevationHigh?: number;
  moistureHigh?: number;
}

export default function init(module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;

export function initSync(module: InitInput): InitOutput;

export function create_grid(params: GenerationParams, width: number, height: number): number;
export function register_biome_definitions(handle: number, definitions: BiomeDefinition[]): boolean;
export function biome_definitions(handle: number): BiomeDefinition[] | null;
export function grid_snapshot(handle: number): GridSnapshot | null;
export function cell_at(handle: number, x: number, y: number): CellData;

type InitInput =
  | string
  | URL
  | Uint8Array
  | Request
  | Response
  | WebAssembly.Module;

type InitOutput = typeof import("./engine");
