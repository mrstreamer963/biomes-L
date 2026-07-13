import biomes from "./biomes.json";

export interface GenerationConditions {
  elevationLt?: number;
  elevationGt?: number;
  elevationGte?: number;
  moistureGt?: number;
}

export interface BiomeDefinition {
  name: string;
  passable: boolean;
  speed: number;
  color: number;
  generation?: GenerationConditions;
}

export interface GenerationParams {
  seed: bigint;
  scale?: number;
  octaves?: number;
  persistence?: number;
  lacunarity?: number;
}

export const DEFAULT_BIOME_DEFINITIONS: BiomeDefinition[] = biomes.definitions;

export const DEFAULT_GENERATION_PARAMS: GenerationParams = {
  ...biomes.generationParams,
  seed: BigInt(biomes.generationParams.seed),
};
