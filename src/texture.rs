use std::{cell::RefCell, rc::Rc};

use crate::{Color, Point3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
#[derive(Clone)]
pub struct SoildColor {
    color: Color,
}

impl Texture for SoildColor {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.color
    }
}

impl SoildColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: Color::new((r, g, b)),
        }
    }
}
#[derive(Clone)]
pub struct CheckerTexture {
    odd: Rc<RefCell<dyn Texture>>,
    even: Rc<RefCell<dyn Texture>>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();
        if sines < 0. {
            return self.odd.borrow().value(u, v, p);
        }
        self.even.borrow().value(u, v, p)
    }
}

impl CheckerTexture {
    pub fn from_colors(color: (Color, Color)) -> Self {
        Self {
            odd: Rc::new(RefCell::new(SoildColor::new(color.0))),
            even: Rc::new(RefCell::new(SoildColor::new(color.1))),
        }
    }
    pub fn new(odd: Rc<RefCell<dyn Texture>>, even: Rc<RefCell<dyn Texture>>) -> Self {
        Self { odd, even }
    }
}
