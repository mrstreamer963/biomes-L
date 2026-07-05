use bevy_ecs::component::Component;

#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct MovementTarget(pub Option<(f64, f64)>);

#[derive(Component, Debug, Clone, Copy)]
pub struct BaseSpeed(pub f64);

#[derive(Component, Debug, Clone, Copy)]
pub struct MovementStatus {
    pub speed_multiplier: f64,
    pub idling: bool,
}

impl Default for MovementStatus {
    fn default() -> Self {
        Self {
            speed_multiplier: 1.0,
            idling: true,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Health(pub u32, pub u32);

#[derive(Component, Debug, Clone, Copy)]
pub struct Team(pub u8);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitKind {
    Scout,
    Soldier,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Selected(pub bool);

#[derive(Component, Debug, Clone)]
pub struct Path(pub Vec<(f64, f64)>);