use std::ops;

use crate::{random_integer_with_range, Point3, Vec3};

const POINT_COUNT: usize = u8::MAX as usize + 1;
#[derive(Clone, Copy)]
pub struct Perlin {
    random_vec: [Vec3; POINT_COUNT],
    perm: (
        [usize; POINT_COUNT],
        [usize; POINT_COUNT],
        [usize; POINT_COUNT],
    ),
}
impl Default for Perlin {
    fn default() -> Self {
        let mut random_vec = [Vec3::default(); POINT_COUNT];
        for i in 0..POINT_COUNT {
            random_vec[i] = Vec3::random_with_range(-1., 1.);
        }
        let perm = (
            Self::generate_perm(),
            Self::generate_perm(),
            Self::generate_perm(),
        );
        Self { random_vec, perm }
    }
}
impl Perlin {
    fn generate_perm() -> [usize; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i;
        }
        Self::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut [usize; POINT_COUNT], n: usize) {
        for i in 0..n {
            let target = random_integer_with_range(0, i as u32) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let (i, j, k) = (
            p.x().floor() as isize,
            p.y().floor() as isize,
            p.z().floor() as isize,
        );
        let mut c = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_vec[self.perm.0[(i + di as isize) as usize & 255]
                        ^ self.perm.1[(j + dj as isize) as usize & 255]
                        ^ self.perm.2[(k + dk as isize) as usize & 255]];
                }
            }
        }
        Self::trilinear_interp(c, u, v, w)
    }

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.;
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new((u - i as f64, v - j as f64, w - k as f64));
                    accum += (i as f64 * uu + (1 - i) as f64 * (1. - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1. - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1. - ww))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }
        accum
    }
    pub fn turb(&self, p: &Point3, depth: usize) -> f64{
        let mut accum = 0.;
        let mut temp_p = p.clone();
        let mut weight = 1.;
        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }
        accum.abs()
    }
}
