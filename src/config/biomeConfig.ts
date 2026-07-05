export interface BiomeDefinition {
  name: string;
  passable: boolean;
  speed: number;
  color: number;
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
  elevationVeryLow?: number;
  elevationVeryHigh?: number;
  elevationSandMax?: number;
}

export const DEFAULT_GENERATION_PARAMS: GenerationParams = {
  seed: 42n,
  scale: 8.0,
  octaves: 4,
  persistence: 0.5,
  lacunarity: 2.0,
  elevationLow: 0.30,
  elevationHigh: 0.70,
  moistureHigh: 0.50,
  elevationVeryLow: 0.28,
  elevationVeryHigh: 0.72,
  elevationSandMax: 0.34,
};

export const DEFAULT_BIOME_DEFINITIONS: BiomeDefinition[] = [
  { name: "Plains", passable: true, speed: 1.0, color: 0x7ec850 },
  { name: "Forest", passable: true, speed: 0.6, color: 0x2d5a27 },
  { name: "Water", passable: false, speed: 0.0, color: 0x3b82f6 },
  { name: "Mountain", passable: false, speed: 0.0, color: 0x8b7355 },
  { name: "Deep Water", passable: false, speed: 0.0, color: 0x1e3a5f },
  { name: "Sand", passable: true, speed: 0.9, color: 0xeedd88 },
  { name: "High Mountain", passable: false, speed: 0.0, color: 0xffffff },
];
