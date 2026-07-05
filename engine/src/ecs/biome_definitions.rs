use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeDef {
    pub name: String,
    pub passable: bool,
    #[serde(rename = "speed")]
    pub speed_factor: f32,
    pub color: u32,
}

#[derive(Debug, Clone)]
pub struct BiomeDefinitions {
    pub definitions: Vec<BiomeDef>,
}

impl BiomeDefinitions {
    pub fn new(definitions: Vec<BiomeDef>) -> Self {
        Self { definitions }
    }

    pub fn get_name(&self, id: u16) -> Option<&str> {
        self.definitions.get(id as usize).map(|d| d.name.as_str())
    }

    pub fn is_passable(&self, id: u16) -> bool {
        self.definitions
            .get(id as usize)
            .map(|d| d.passable)
            .unwrap_or(false)
    }

    pub fn speed_factor(&self, id: u16) -> f32 {
        self.definitions
            .get(id as usize)
            .map(|d| d.speed_factor)
            .unwrap_or(0.0)
    }

    pub fn get_color(&self, id: u16) -> u32 {
        self.definitions
            .get(id as usize)
            .map(|d| d.color)
            .unwrap_or(0x000000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_defs() -> BiomeDefinitions {
        BiomeDefinitions::new(vec![
            BiomeDef {
                name: "Plains".into(),
                passable: true,
                speed_factor: 1.0,
                color: 0x7ec850,
            },
            BiomeDef {
                name: "Forest".into(),
                passable: true,
                speed_factor: 0.6,
                color: 0x2d5a27,
            },
            BiomeDef {
                name: "Water".into(),
                passable: false,
                speed_factor: 0.0,
                color: 0x3b82f6,
            },
            BiomeDef {
                name: "Mountain".into(),
                passable: false,
                speed_factor: 0.0,
                color: 0x8b7355,
            },
            BiomeDef {
                name: "Deep Water".into(),
                passable: false,
                speed_factor: 0.0,
                color: 0x1e3a5f,
            },
            BiomeDef {
                name: "Sand".into(),
                passable: true,
                speed_factor: 0.9,
                color: 0xeedd88,
            },
            BiomeDef {
                name: "High Mountain".into(),
                passable: false,
                speed_factor: 0.0,
                color: 0xffffff,
            },
        ])
    }

    #[test]
    fn get_name_returns_some_for_valid_id() {
        let defs = test_defs();
        assert_eq!(defs.get_name(0), Some("Plains"));
        assert_eq!(defs.get_name(1), Some("Forest"));
        assert_eq!(defs.get_name(2), Some("Water"));
        assert_eq!(defs.get_name(3), Some("Mountain"));
        assert_eq!(defs.get_name(4), Some("Deep Water"));
        assert_eq!(defs.get_name(5), Some("Sand"));
        assert_eq!(defs.get_name(6), Some("High Mountain"));
    }

    #[test]
    fn get_name_returns_none_for_invalid_id() {
        let defs = test_defs();
        assert_eq!(defs.get_name(99), None);
    }

    #[test]
    fn is_passable_works() {
        let defs = test_defs();
        assert!(defs.is_passable(0));
        assert!(defs.is_passable(1));
        assert!(!defs.is_passable(2));
        assert!(!defs.is_passable(3));
        assert!(!defs.is_passable(4));
        assert!(defs.is_passable(5));
        assert!(!defs.is_passable(6));
    }

    #[test]
    fn is_passable_returns_false_for_invalid_id() {
        let defs = test_defs();
        assert!(!defs.is_passable(99));
    }

    #[test]
    fn speed_factor_works() {
        let defs = test_defs();
        assert_eq!(defs.speed_factor(0), 1.0);
        assert_eq!(defs.speed_factor(1), 0.6);
        assert_eq!(defs.speed_factor(2), 0.0);
        assert_eq!(defs.speed_factor(3), 0.0);
        assert_eq!(defs.speed_factor(4), 0.0);
        assert_eq!(defs.speed_factor(5), 0.9);
        assert_eq!(defs.speed_factor(6), 0.0);
    }

    #[test]
    fn speed_factor_returns_zero_for_invalid_id() {
        let defs = test_defs();
        assert_eq!(defs.speed_factor(99), 0.0);
    }

    #[test]
    fn get_color_works() {
        let defs = test_defs();
        assert_eq!(defs.get_color(0), 0x7ec850);
        assert_eq!(defs.get_color(1), 0x2d5a27);
        assert_eq!(defs.get_color(2), 0x3b82f6);
        assert_eq!(defs.get_color(3), 0x8b7355);
        assert_eq!(defs.get_color(4), 0x1e3a5f);
        assert_eq!(defs.get_color(5), 0xeedd88);
        assert_eq!(defs.get_color(6), 0xffffff);
    }

    #[test]
    fn get_color_returns_black_for_invalid_id() {
        let defs = test_defs();
        assert_eq!(defs.get_color(99), 0x000000);
    }
}
