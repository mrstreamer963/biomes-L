//! Biome type and its gameplay attributes.
//!
//! `Biome` is a `#[repr(u8)]` enum so that a grid snapshot can be a flat
//! `Vec<u8>` of biome indices, trivially handed to JS as a `Uint8Array`.
//! Attributes (`passable`, `speed_factor`) live in `match` functions rather
//! than in enum fields, keeping the type plain data.

/// Kinds of biome a cell can hold.
///
/// Discriminant values are part of the snapshot wire format — do not
/// renumber existing variants.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Biome {
    Plains = 0,
    Forest = 1,
    Water = 2,
    Mountain = 3,
}

impl Biome {
    /// Whether a unit can enter this biome at all.
    pub fn passable(self) -> bool {
        match self {
            Biome::Plains | Biome::Forest => true,
            Biome::Water | Biome::Mountain => false,
        }
    }

    /// Speed multiplier for traversing this biome. Only meaningful when
    /// [`passable`](Self::passable) is `true`.
    pub fn speed_factor(self) -> f32 {
        match self {
            Biome::Plains => 1.0,
            Biome::Forest => 0.6,
            // Impassable biomes have no traversal speed; report 0.0.
            Biome::Water | Biome::Mountain => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passable_plains_and_forest() {
        assert!(Biome::Plains.passable());
        assert!(Biome::Forest.passable());
    }

    #[test]
    fn impassable_water_and_mountain() {
        assert!(!Biome::Water.passable());
        assert!(!Biome::Mountain.passable());
    }

    #[test]
    fn speed_factors_in_range() {
        assert_eq!(Biome::Plains.speed_factor(), 1.0);
        assert_eq!(Biome::Forest.speed_factor(), 0.6);
    }

    #[test]
    fn discriminants_are_stable() {
        assert_eq!(Biome::Plains as u8, 0);
        assert_eq!(Biome::Forest as u8, 1);
        assert_eq!(Biome::Water as u8, 2);
        assert_eq!(Biome::Mountain as u8, 3);
    }
}
