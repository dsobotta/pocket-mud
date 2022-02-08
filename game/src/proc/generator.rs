use rand::rngs::StdRng;
use crate::worldcell::{
    Region,
    WorldCell
};

pub trait Generator {
    fn generate(&self, rand: &mut StdRng, in_world: &WorldCell, out_world: &mut WorldCell, region: &Region);
}