/* tslint:disable */
/* eslint-disable */

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

export interface UnitData {
  id: number;
  x: number;
  y: number;
  health: number;
  max_health: number;
  unit_type: string;
  team: number;
  selected: boolean;
  debug: boolean;
  path: [number, number][];
  target: [number, number] | null;
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

export function biome_definitions(handle: number): any;
export function cell_at(handle: number, x: number, y: number): any;
export function create_grid(params: any, width: number, height: number): number;
export function create_unit(handle: number, x: number, y: number, unit_type: string): number;
export function grid_snapshot(handle: number): any;
export function register_biome_definitions(handle: number, definitions: any): boolean;
export function set_unit_debug(handle: number, unit_id: number, debug: boolean): void;
export function set_unit_selected(handle: number, unit_id: number, selected: boolean): void;
export function set_unit_target(handle: number, unit_id: number, x: number, y: number): void;
export function spawn_starting_units(handle: number): void;
export function tick(handle: number, dt: number): any;
export function unit_count(handle: number): number;

type InitInput =
  | string
  | URL
  | Uint8Array
  | Request
  | Response
  | WebAssembly.Module;

type InitOutput = typeof import("./engine");