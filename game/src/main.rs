#![no_std]

mod worldcell;
mod tile;
mod render;
mod proc;

use worldcell::*;
use render::Renderer;

use proc::noise::NoiseGenerator;
use proc::generator::Generator;
use proc::cell::CellGenerator;
use proc::concrete::ConcreteGenerator;

use rand::{rngs::StdRng, SeedableRng};

fn main() {

    // let seed: u64 = 1;
    // let seed: u64 = 3294872943;
    // let seed: u64 = 4524352;
    // let seed: u64 = 24362262;
    let seed: u64 = 4573753643;
    // let seed: u64 = 245245245;

    
    let mut rand = StdRng::seed_from_u64(seed);

    let noise_gen = NoiseGenerator::new(0.50);

    //score: 8 - maze - traditional
    // let cell_gen = CellGenerator::new_with_thresholds(1, 1, 0, 3);

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

    //score: 10 - forest, mixed, clearings, clusters
    let cell_gen = CellGenerator::new_with_thresholds( 0, 1, 3, 4);


    let concrete_gen = ConcreteGenerator::new();


    let mut world_a = WorldCell::new();
    let mut world_b = WorldCell::new();

    let curr_world = &mut world_a;
    let mut next_world = &mut world_b;


    let region = Region{x:0, y:0, width:worldcell::MAX_WIDTH-1, height:worldcell::MAX_HEIGHT-1};
    noise_gen.generate(&mut rand, curr_world, &mut next_world, &region);
    core::mem::swap(curr_world, next_world);


    let region = Region{x:1, y:1, width:worldcell::MAX_WIDTH-3, height:worldcell::MAX_HEIGHT-3};
    let mut renderer = Renderer::new();

    //BOOTSTRAP + GENERATE
    for _i in 0..15 {
        cell_gen.generate(&mut rand, curr_world, &mut next_world, &region);
        core::mem::swap(curr_world, next_world);
    }

    concrete_gen.generate(&mut rand, curr_world, &mut next_world, &region);
    core::mem::swap(curr_world, next_world);

 
    let mut player_pos_x = 14;
    let mut player_pos_y = 0;
    let ticks_between_renders = 250;
    let mut curr_ticks = 0;
    loop {
        curr_ticks += 1;
        if curr_ticks < ticks_between_renders {
            let mut x = f32::sqrt(curr_ticks as f32);
            for _i in 0..90210 {
                x = f32::sqrt(x);
            }

            continue;
        }   

        curr_ticks = 0;
        player_pos_x += 1;
        player_pos_y += 1;
        renderer.set_focus(player_pos_x, player_pos_y);
        let _result = renderer.render(curr_world);
    }
}