use super::generator::Generator;

use crate::tile::TileType;
use crate::worldcell::{
    Region,
    WorldCell
};
use rand::rngs::StdRng;

pub struct ConcreteGenerator {

}

impl ConcreteGenerator {

    pub fn new() -> ConcreteGenerator {
        ConcreteGenerator {}
    }
}

impl Generator for ConcreteGenerator {

    fn generate(&self, _rand: &mut StdRng, in_world: &WorldCell, out_world: &mut WorldCell, region: &Region)  {
        out_world.validate_region(region);
        *out_world = *in_world;

        let beg_x = region.x;
        let beg_y = region.y;
        let end_x = beg_x + region.width;
        let end_y = beg_y + region.height;

        for y in beg_y..end_y {
            for x in beg_x..end_x {

                let exists = TileType::GenWall == in_world.read(x, y);

                if !exists {
                    out_world.write(x, y, TileType::None);
                    continue;
                }

                let n = TileType::GenWall == in_world.read(x, y-1);
                let s = TileType::GenWall == in_world.read(x, y+1);
                let w = TileType::GenWall == in_world.read(x-1, y);
                let e = TileType::GenWall == in_world.read(x+1, y);

                let mut count = 0;
                if n { count += 1; }
                if s { count += 1; }
                if w { count += 1; }
                if e { count += 1; }
                

                let c_tile =
                if 4 == count {
                    TileType::WallFour
                } else if 3 == count {
                    if n && s && w {
                        TileType::WallThreeW
                    } else if n && s && e {
                        TileType::WallThreeE
                    } else if w && e && n {
                        TileType::WallThreeN
                    } else {
                        TileType::WallThreeS
                    }
                } else if 2 == count {
                    if n && s {
                        TileType::WallVertical
                    } else if w && e {
                        TileType::WallHorizontal
                    } else if n && w {
                        TileType::WallCornerNW
                    } else if n && e {
                        TileType::WallCornerNE
                    } else if s && w {
                        TileType::WallCornerSW
                    } else {
                        TileType::WallCornerSE
                    }
                } else if 1 == count {
                    // if n {
                    //     TileType::WallCapN
                    // } else if s {
                    //     TileType::WallCapS
                    // } else if w {
                    //     TileType::WallCapW
                    // } else {
                    //     TileType::WallCapE
                    // }
                    if n || s {
                        TileType::WallVertical
                    } else {
                        TileType::WallHorizontal
                    }
                } else {
                    TileType::WallIsland
                };

                out_world.write(x, y, c_tile);
            }
        }
    }
}