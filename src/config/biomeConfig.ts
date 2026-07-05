export interface BiomeDefinition {
  name: string;
  passable: boolean;
  speed: number;
  color: number;
}

export const DEFAULT_BIOME_DEFINITIONS: BiomeDefinition[] = [
  { name: "Plains", passable: true, speed: 1.0, color: 0x7ec850 },
  { name: "Forest", passable: true, speed: 0.6, color: 0x2d5a27 },
  { name: "Water", passable: false, speed: 0.0, color: 0x3b82f6 },
  { name: "Mountain", passable: false, speed: 0.0, color: 0x8b7355 },
];
