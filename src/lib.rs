

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
    thread_rng().gen()
}
//7-1 just talk about some c++ thing, c++ use random library
//in rust we don't have it in standard library, I choose rand here.
pub mod camera;
pub use camera::*;