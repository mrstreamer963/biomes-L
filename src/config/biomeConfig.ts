import biomes from "./biomes.json";

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

export const DEFAULT_BIOME_DEFINITIONS: BiomeDefinition[] = biomes.definitions;

export const DEFAULT_GENERATION_PARAMS: GenerationParams = {
  ...biomes.generationParams,
  seed: BigInt(biomes.generationParams.seed),
};
