use std::sync::Arc;

use image::RgbImage;

use crate::{Color, Perlin, Point3};

pub trait Texture : Send + Sync{
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
#[derive(Clone)]
pub struct SolidColor {
    color: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color
    }
}

impl SolidColor {
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
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();
        if sines < 0. {
            return self.odd.value(u, v, p);
        }
        self.even.value(u, v, p)
    }
}

impl CheckerTexture {
    pub fn from_colors(color: (Color, Color)) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(color.0)),
            even: Arc::new(SolidColor::new(color.1)),
        }
    }
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self { odd, even }
    }
}
#[derive(Default, Clone, Copy)]
pub struct NoiseTexture {
    pub(crate) noise: Perlin,
    pub(crate) scale: f64,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new((1., 1., 1.)) * 0.5 * (1. + (self.scale * p.z() + 10. * self.noise.turb(p, 7)).sin())
    }
}
impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }
}
#[derive(Default, Clone)]
pub struct ImageTexture {
    data: RgbImage,
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: &Point3) -> Color {
        if self.data.is_empty() {
            return Color::new((0., 1., 1.));
        }
        u = u.clamp(0., 1.);
        v = 1. - v.clamp(0., 1.);
        let (width, height) = self.data.dimensions();
        let (mut i, mut j) = ((width as f64 * u) as u32, (height as f64 * v) as u32);
        if i >= width {
            i = width - 1;
        }
        if j >= height {
            j = height - 1;
        }
        const COLOR_SCALE: f64 = 1. / 255.;
        let pixel = self.data.get_pixel(i, j);
        
        Color::new((COLOR_SCALE * pixel[0] as f64, COLOR_SCALE * pixel[1] as f64, COLOR_SCALE * pixel[2] as f64))
    }
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        match image::open(filename){
            Ok(img) => return Self { 
                data: img.to_rgb8()
            },
            Err(err) => panic!("{:?}",err),
        };
        
    }
}