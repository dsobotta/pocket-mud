#![no_std]

mod worldcell;
mod tile;
mod render;
mod proc;

use worldcell::WorldCell;
use worldcell::Region;
use render::Renderer;

use proc::noise::NoiseGenerator;
use proc::generator::Generator;
use proc::cellgenerator::CellGenerator;

use rand::{rngs::StdRng, SeedableRng};

fn main() -> Result<(), core::convert::Infallible> {

     let seed: u64 = 1;
    // let seed: u64 = 3294872943;
    // let seed: u64 = 4524352;
    // let seed: u64 = 24362262;
    //let seed: u64 = 4573753643;
    // let seed: u64 = 245245245;

    
    let mut rand = StdRng::seed_from_u64(seed);

    let noise_gen = NoiseGenerator::new(0.55);
    let cell_gen = CellGenerator::new();


    let mut world_a = WorldCell::new();
    let mut world_b = WorldCell::new();

    let region = Region{x:1, y:1, width:40, height:40};

    noise_gen.generate(&mut rand, &world_a, &mut world_b, &region);


    cell_gen.generate(&mut rand, &world_b, &mut world_a, &region);
    cell_gen.generate(&mut rand, &world_a, &mut world_b, &region);
    cell_gen.generate(&mut rand, &world_b, &mut world_a, &region);
    cell_gen.generate(&mut rand, &world_a, &mut world_b, &region);
    cell_gen.generate(&mut rand, &world_b, &mut world_a, &region);
    cell_gen.generate(&mut rand, &world_b, &mut world_a, &region);
    cell_gen.generate(&mut rand, &world_a, &mut world_b, &region);
    cell_gen.generate(&mut rand, &world_b, &mut world_a, &region);
    cell_gen.generate(&mut rand, &world_a, &mut world_b, &region);
    // cell_gen.generate(&mut rand, &world_b, &mut world_a, &region);
    // cell_gen.generate(&mut rand, &world_a, &mut world_b, &region);
    // cell_gen.generate(&mut rand, &world_b, &mut world_a, &region);
    // cell_gen.generate(&mut rand, &world_a, &mut world_b, &region);
    // cell_gen.generate(&mut rand, &world_b, &mut world_a, &region);
    // cell_gen.generate(&mut rand, &world_b, &mut world_a, &region);

    let mut renderer = Renderer::new();

    //loop {
    {
        let result: Result<(), core::convert::Infallible> = renderer.render(&world_a);

        match result {
            Ok(_v) => {},
            Err(_e) => {},
        }
    }

    Ok(())
}