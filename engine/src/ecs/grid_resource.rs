use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Resource;

use crate::rng::Lcg;

#[derive(Debug, Resource)]
pub struct GridResource {
    pub width: u32,
    pub height: u32,
    pub entities: Vec<Entity>,
    pub rng: Lcg,
}

impl GridResource {
    pub fn new(width: u32, height: u32, entities: Vec<Entity>, rng: Lcg) -> Self {
        Self {
            width,
            height,
            entities,
            rng,
        }
    }
}
