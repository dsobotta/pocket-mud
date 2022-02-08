use super::generator::Generator;

use crate::tile::TileType;
use crate::worldcell::{
    Region,
    WorldCell
};

use rand::{rngs::StdRng, Rng};

pub struct NoiseGenerator {
    density: f32, //0-1 for 0-100%
}

impl NoiseGenerator {
    pub fn new(density: f32) -> NoiseGenerator {
        NoiseGenerator {
            density
        }
    }
}

impl Generator for NoiseGenerator {
    fn generate(&self, rand: &mut StdRng, in_world: &WorldCell, out_world: &mut WorldCell, region: &Region) {
        out_world.validate_region(region);
        *out_world = *in_world;

        let beg_x = region.x;
        let beg_y = region.y;
        let end_x = beg_x + region.width;
        let end_y = beg_y + region.height;
        for y in beg_y..end_y {
            for x in beg_x..end_x {
                let val: f32 = rand.gen_range(0.0, 1.0);
                if val < self.density {
                    out_world.tiles[x][y] = TileType::GenWall;
                } else {
                    out_world.tiles[x][y] = TileType::GenFloor;
                }
            }
        }
    }
}