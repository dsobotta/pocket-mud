use super::generator::Generator;

use crate::tile::TileType;
use crate::worldcell::{
    Region,
    WorldCell
};
use rand::rngs::StdRng;
pub struct CellGenerator {
    low_spawn: u8,
    high_spawn: u8,
    low_keep: u8,
    high_keep: u8,
}

impl CellGenerator {

    // pub fn new() -> CellGenerator {
    //     CellGenerator::new_with_thresholds(0, 0, 4, 6)
    // }

    pub fn new_with_thresholds(spawn_low: u8, spawn_high: u8, keep_low: u8, keep_high: u8) -> CellGenerator {
        CellGenerator {
            low_spawn: spawn_low,
            high_spawn: spawn_high,
            low_keep: keep_low,
            high_keep: keep_high,
        }
    }
}

impl Generator for CellGenerator {

    fn generate(&self, _rand: &mut StdRng, in_world: &WorldCell, out_world: &mut WorldCell, region: &Region)  {
        out_world.validate_region(region);
        *out_world = *in_world;

        let beg_x = region.x;
        let beg_y = region.y;
        let end_x = beg_x + region.width;
        let end_y = beg_y + region.height;

        for y in beg_y..end_y {
            for x in beg_x..end_x {

                let count =
                    in_world.read(x-1, y-1) as u8 +
                    in_world.read(x, y-1) as u8 +
                    in_world.read(x+1, y-1) as u8 +

                    in_world.read(x-1, y) as u8 +
                    in_world.read(x+1, y) as u8 +

                    in_world.read(x-1, y+1) as u8 +
                    in_world.read(x, y+1) as u8 +
                    in_world.read(x+1, y+1) as u8
                ;

                let exists = TileType::GenWall == in_world.read(x, y);
                let should_spawn = self.low_spawn <= count && count <= self.high_spawn;
                let should_keep = self.low_keep <= count && count <= self.high_keep;

                if (!exists && should_spawn) || (exists && should_keep) {
                    out_world.write(x, y, TileType::GenWall);
                } else {
                    out_world.write(x, y, TileType::GenFloor);
                }
            }
        }
    }
}