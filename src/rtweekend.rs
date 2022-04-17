use rand::prelude::*;

pub fn random_float(min: f32, max: f32) -> f32 {
    thread_rng().gen_range(min..max)
}
