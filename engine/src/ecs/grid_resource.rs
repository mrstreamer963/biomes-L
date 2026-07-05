#[derive(Debug)]
pub struct GridResource {
    pub width: u32,
    pub height: u32,
    pub biome_ids: Vec<u16>,
    pub resources: Vec<u8>,
}

impl GridResource {
    pub fn new(width: u32, height: u32, biome_ids: Vec<u16>, resources: Vec<u8>) -> Self {
        Self {
            width,
            height,
            biome_ids,
            resources,
        }
    }
}
