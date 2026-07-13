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

// Городские зоны (ID 0–6). Генерация: elevation = плотность застройки, moisture = запутанность района.
export const DEFAULT_BIOME_DEFINITIONS: BiomeDefinition[] = [
  { name: "Улица", passable: true, speed: 1.0, color: 0x4a4a4a },
  { name: "Переулок", passable: true, speed: 0.6, color: 0x2a2a35 },
  { name: "Канал", passable: false, speed: 0.0, color: 0x2b5f8a },
  { name: "Здание", passable: false, speed: 0.0, color: 0x6b6b6b },
  { name: "Гавань", passable: false, speed: 0.0, color: 0x1a3a5c },
  { name: "Стройплощадка", passable: true, speed: 0.9, color: 0xc4a35a },
  { name: "Небоскрёб", passable: false, speed: 0.0, color: 0x1e1e28 },
];
