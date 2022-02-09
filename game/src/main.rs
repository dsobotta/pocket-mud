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

fn main() {

    let seed: u64 = 1;
    let seed: u64 = 3294872943;
    let seed: u64 = 4524352;
    let seed: u64 = 24362262;
    let seed: u64 = 4573753643;
    // let seed: u64 = 245245245;

    
    let mut rand = StdRng::seed_from_u64(seed);

    let noise_gen = NoiseGenerator::new(0.50);

    //score: 8 - maze - traditional
    let cell_gen = CellGenerator::new_with_thresholds(1, 1, 0, 3);

    //score: 7 - cave - mixed
    // let cell_gen = CellGenerator::new_with_thresholds( 1, 2, 5, 5);


    //score: 8 - ruins - mixed building and rubble size
    // let cell_gen = CellGenerator::new_with_thresholds( 0, 0, 4, 5);

    //score: 9 - sparse cave
    // let cell_gen = CellGenerator::new_with_thresholds( 1, 1, 6, 6);


    //score: 9 - cave - winding, open
    // let cell_gen = CellGenerator::new_with_thresholds( 1, 1, 6, 7);


    //score: 10 - cave - dense, rooms, tunnels
    // let cell_gen = CellGenerator::new_with_thresholds( 1, 2, 6, 7);

    //score: 8 - forest - dense, uniform
    // let cell_gen = CellGenerator::new_with_thresholds( 0, 0, 0, 3);

    //score: 6 - canyon, debris, sometimes impassable
    // let cell_gen = CellGenerator::new_with_thresholds( 1, 2, 1, 2);

    //score: 10 - cave/canyon, clean paths, always connects 2 or more quadrants
    // let cell_gen = CellGenerator::new_with_thresholds( 1, 4, 1, 2);

    //score: 10 - great mixed forest, clearings and clusters!
    // let cell_gen = CellGenerator::new_with_thresholds( 0, 1, 3, 4);


    let mut world_a = WorldCell::new();
    let mut world_b = WorldCell::new();

    let region = Region{x:1, y:1, width:40, height:40};

    noise_gen.generate(&mut rand, &world_a, &mut world_b, &region);

    let mut renderer = Renderer::new();

    let mut cycle_a = false;
    let mut num_generations = 0;
    let bootstrap_generations = 15;


    let mut curr_ticks = 0;
    let ticks_between_renders = 5000;

    loop {
        curr_ticks += 1;
        if (num_generations >= bootstrap_generations) {
            if curr_ticks < ticks_between_renders {
                let mut x = f32::sqrt(curr_ticks as f32);
                for _i in 0..90210 {
                    x = f32::sqrt(x);
                }

                continue;
            }   
        }

        num_generations += 1;
        curr_ticks = 0;
        cycle_a = !cycle_a;

        if cycle_a {
            cell_gen.generate(&mut rand, &world_b, &mut world_a, &region);
            let _result = renderer.render(&world_a);
        } else {
            cell_gen.generate(&mut rand, &world_a, &mut world_b, &region);
            let _result = renderer.render(&world_b);
        }

    }
}