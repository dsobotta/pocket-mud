use crate::tile::TileType;

const MAX_WIDTH: usize = 256;
const MAX_HEIGHT: usize = 128;
const MIN_ROOMS: u16 = 4;
const MAX_ROOMS: u16 = 10;

#[derive(Copy, Clone)]
pub struct Region {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}


#[derive(Copy, Clone)]
pub struct WorldCell {
    pub tiles: [[TileType; MAX_HEIGHT]; MAX_WIDTH],
}

impl WorldCell {

    pub fn new() -> WorldCell {
        WorldCell { tiles: [[TileType::GenWall; MAX_HEIGHT]; MAX_WIDTH] }
    }

    pub fn validate_region(&self, region: &Region) {
        assert!( (region.x + region.width) < MAX_WIDTH);
        assert!( (region.y + region.height) < MAX_HEIGHT);
    }

    pub fn clear(&mut self) {
        self.tiles = [[TileType::GenWall; MAX_HEIGHT]; MAX_WIDTH];
    }
}