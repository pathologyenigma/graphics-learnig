

pub mod vec;
pub mod ray;
pub mod hittable;
pub use vec::*;
pub use ray::*;
pub use hittable::*;
//6-6 just talk about some c++ thing, In rust we could use
//the Rc<RefCell<T>> to implement as well.
//just some memory safety problem.
//which will not happened in rust.

pub use std::f64::consts::PI as PI;
pub const INFINITY: f64 = f64::INFINITY;
pub use rand::prelude::*;

pub fn random_float() -> f64 {
    thread_rng().gen_range(0.0..1.)
}
pub fn random_float_with_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}
pub fn degree_to_radians(d: f64) -> f64 {
    d * PI / 180.
}
//7-1 just talk about some c++ thing, c++ use random library
//in rust we don't have it in standard library, I choose rand here.
pub mod camera;
pub use camera::*;
pub mod material;
pub use material::*;
pub mod aabb;
pub use aabb::*;
pub mod bvh;
pub use bvh::*;
pub fn random_integer_with_range(min: u32, max: u32) -> u32 { 
    thread_rng().gen_range(min..max+1)
} 