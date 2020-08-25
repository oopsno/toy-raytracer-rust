use crate::color::Color;
pub use crate::math::{floats, random, random_between, Float};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl From<Float> for Vec3 {
    fn from(scalar: Float) -> Self {
        Vec3 {
            x: scalar,
            y: scalar,
            z: scalar,
        }
    }
}

impl From<(Float, Float, Float)> for Vec3 {
    fn from(xyz: (Float, Float, Float)) -> Self {
        Vec3 {
            x: xyz.0,
            y: xyz.1,
            z: xyz.2,
        }
    }
}

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Vec3 { x, y, z }
    }

    pub fn random() -> Self {
        let x = random();
        let y = random();
        let z = random();
        Vec3::new(x, y, z)
    }

    pub fn random_between(min: Float, max: Float) -> Self {
        let x = random_between(min, max);
        let y = random_between(min, max);
        let z = random_between(min, max);
        Vec3::new(x, y, z)
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_between(-1., 1.);
            if p.length() < 1. {
                return p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let x = random_between(-1.0, 1.0);
            let y = random_between(-1.0, 1.0);
            let p = Vec3::new(x, y, 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let p = Self::random_in_unit_sphere();
        if p.dot(normal) > 0. {
            p
        } else {
            -p
        }
    }

    pub fn random_unit_vector() -> Self {
        let t = random_between(0., 2. * floats::consts::PI);
        let z = random_between(-1., 1.);
        let r = (1. - z.powi(2)).sqrt();
        Vec3::new(r * t.cos(), r * t.sin(), z)
    }

    pub fn zeros() -> Self {
        Vec3::from(0 as Float)
    }

    pub fn ones() -> Self {
        Vec3::from(1 as Float)
    }

    pub fn apply<F>(&self, f: F) -> Self
    where
        F: Fn(Float) -> Float,
    {
        Vec3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    pub fn powi(&self, n: i32) -> Self {
        self.apply(|x| x.powi(n))
    }

    pub fn sum(&self) -> Float {
        self.x + self.y + self.z
    }

    pub fn length_squared(&self) -> Float {
        self.dot(self)
    }

    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Self) -> Float {
        self.element_wise(Float::mul, other).sum()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit(&self) -> Self {
        self.element_wise_by(Float::div, self.length())
    }

    pub fn element_wise<F>(&self, f: F, other: &Self) -> Self
    where
        F: Fn(Float, Float) -> Float,
    {
        Self {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
            z: f(self.z, other.z),
        }
    }

    pub fn element_wise_by<F>(&self, f: F, other: Float) -> Self
    where
        F: Fn(Float, Float) -> Float,
    {
        Self {
            x: f(self.x, other),
            y: f(self.y, other),
            z: f(self.z, other),
        }
    }

    /// 计算反射光线的方向
    pub fn reflect(&self, normal: &Vec3) -> Self {
        *self - *normal * (2.0 * self.dot(&normal))
    }

    /// 计算折射光线的方向
    pub fn refract(&self, normal: &Vec3, etai_over_etat: Float) -> Self {
        let cos_theta = (-self.dot(normal)).min(1.0);
        let r_out_perp = (*self + *normal * cos_theta) * etai_over_etat;
        let r_out_parallel = *normal * -((1. - r_out_perp.length_squared()).abs().sqrt());
        r_out_perp + r_out_parallel
    }

    pub fn into_color(self) -> Color {
        Color::rgb(self.x, self.y, self.z)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::zeros()
    }
}

use std::ops::{Add, Div, Mul, Neg, Sub};

macro_rules! element_wise_op {
    ($typename:ident, $opname:ident, $fnname:ident) => {
        impl $opname for $typename {
            type Output = Self;

            fn $fnname(self, other: Self) -> Self {
                self.element_wise(Float::$fnname, &other)
            }
        }

        impl $opname<Float> for $typename {
            type Output = Self;

            fn $fnname(self, other: Float) -> Self {
                self.element_wise_by(Float::$fnname, other)
            }
        }

        impl $opname<Vec3> for Float {
            type Output = Vec3;

            fn $fnname(self, other: Vec3) -> Vec3 {
                Vec3::from(self).$fnname(other)
            }
        }
    };
}

element_wise_op!(Vec3, Add, add);
element_wise_op!(Vec3, Sub, sub);
element_wise_op!(Vec3, Mul, mul);
element_wise_op!(Vec3, Div, div);

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        -1. as Float * self
    }
}

#[test]
fn test_vec3() {
    let u = Vec3::random();
    let v = Vec3::random();
    assert_eq!(v.dot(&v), v.length_squared());
    assert_eq!(u.dot(&-v), -u.dot(&v));
}

#[test]
fn test_reflect() {
    let x = Vec3::new(1., -1., 0.);
    let normal = Vec3::new(0., 1., 0.);
    let reflected = Vec3::new(1., 1., 0.);
    assert_eq!(x.reflect(&normal).unit(), reflected.unit());
}

#[test]
fn test_refract() {
    let x = Vec3::new(1., -1., 0.).unit();
    let normal = Vec3::new(0., 1., 0.);
    let refracted = Vec3::new(1., -(3. as Float).sqrt(), 0.);
    println!("x = {:?}, n = {:?}, r = {:?}", x, normal, refracted);
    assert_eq!(
        x.refract(&normal, (2 as Float).sqrt() / 2.).unit(),
        refracted.unit()
    );
}
