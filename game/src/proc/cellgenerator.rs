use super::generator::Generator;

use crate::tile::TileType;
use crate::worldcell::{
    Region,
    WorldCell
};
use rand::rngs::StdRng;

pub struct CellGenerator;

impl CellGenerator {
    pub fn new() -> CellGenerator {
        CellGenerator
    }
}

impl Generator for CellGenerator {

    fn generate(&self, rand: &mut StdRng, in_world: &WorldCell, out_world: &mut WorldCell, region: &Region)  {
        out_world.validate_region(region);
        *out_world = *in_world;

        let beg_x = region.x + 1;
        let beg_y = region.y + 1;
        let end_x = beg_x + region.width - 2;
        let end_y = beg_y + region.height - 2;
        for y in beg_y..end_y {
            for x in beg_x..end_x {

                let mut neighbors = 0;
                if TileType::GenWall == in_world.tiles[x-1][y-1] { neighbors += 1; }
                if TileType::GenWall == in_world.tiles[x][y-1] { neighbors += 1; }
                if TileType::GenWall == in_world.tiles[x+1][y-1] { neighbors += 1; }
                
                if TileType::GenWall == in_world.tiles[x-1][y] { neighbors += 1; }
                if TileType::GenWall == in_world.tiles[x+1][y] { neighbors += 1; }

                if TileType::GenWall == in_world.tiles[x-1][y+1] { neighbors += 1; }
                if TileType::GenWall == in_world.tiles[x][y+1] { neighbors += 1; }
                if TileType::GenWall == in_world.tiles[x+1][y+1] { neighbors += 1; }

                if 0 == neighbors || 4 < neighbors {
                    out_world.tiles[x][y] = TileType::GenWall;
                } else {
                    out_world.tiles[x][y] = TileType::GenFloor;
                }
            }
        }
    }
}