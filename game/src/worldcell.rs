use crate::tile::TileType;

pub const MAX_WIDTH: u16 = 240;
pub const MAX_HEIGHT: u16 = 240;
const TILE_ARRAY_SIZE: u16 = (MAX_WIDTH+2)*(MAX_HEIGHT+2);

#[derive(Copy, Clone)]
pub struct Region {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}



#[derive(Copy, Clone)]
pub struct WorldCell {
    tiles: [TileType; TILE_ARRAY_SIZE as usize],
}

#[allow(dead_code)]
impl WorldCell {

    pub fn new() -> WorldCell {
        WorldCell { tiles: [TileType::GenWall; TILE_ARRAY_SIZE as usize], }
    }

    pub fn validate_region(&self, region: &Region) {
        assert!( (region.x + region.width) < MAX_WIDTH);
        assert!( (region.y + region.height) < MAX_HEIGHT);
    }

    pub fn clear(&mut self) {
        self.tiles = [TileType::GenWall; TILE_ARRAY_SIZE as usize];
    }

    pub fn read(&self, x: u16, y: u16) -> TileType {
        let idx = (MAX_WIDTH+2)*y + x;
        return self.tiles[idx as usize];
    }

    pub fn write(&mut self, x: u16, y:u16, value: TileType) {
        let idx = (MAX_WIDTH+2)*y + x;
        self.tiles[idx as usize] = value;
    }
}