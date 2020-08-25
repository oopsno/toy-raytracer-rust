use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::math::{random, Float};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Scatter {
    /// 衰减
    pub attenuation: Vec3,
    /// 散射光线
    pub ray: Ray,
}

pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub type AnyMaterial = Arc<dyn Material>;

/// 博朗材质
#[derive(Debug, Clone)]
pub struct Lambertian {
    /// 反照率
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(r: Float, g: Float, b: Float) -> Lambertian {
        Lambertian {
            albedo: Vec3::new(r, g, b),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let direction = rec.normal + Vec3::random_unit_vector();
        Some(Scatter {
            attenuation: self.albedo,
            ray: Ray::new(rec.p, direction),
        })
    }
}

/// 金属材质
#[derive(Debug, Clone)]
pub struct Metal {
    /// 反照率
    pub albedo: Vec3,
    pub fuzz: Float,
}

impl Metal {
    pub fn new(r: Float, g: Float, b: Float, fuzz: Float) -> Metal {
        Metal {
            albedo: Vec3::new(r, g, b),
            fuzz: fuzz.min(1.0).max(0.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let direction = ray.direction.unit().reflect(&rec.normal);
        if direction.dot(&rec.normal) > 0. {
            Some(Scatter {
                attenuation: self.albedo,
                ray: Ray::new(rec.p, direction + Vec3::random_in_unit_sphere() * self.fuzz),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    ref_idx: Float,
}

impl Dielectric {
    pub fn new(ri: Float) -> Self {
        Dielectric { ref_idx: ri }
    }

    fn schlick(cosine: Float, ref_idx: Float) -> Float {
        let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let etai_over_etat = if rec.front_face {
            1. / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = ray.direction.unit();
        let cos_theta = (-unit_direction).dot(&rec.normal);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();
        let direction = if etai_over_etat * sin_theta > 1. {
            // 由光密介质进入光疏介质且入射角大于临界值时发生全反射
            unit_direction.reflect(&rec.normal)
        } else {
            // 否则由菲涅尔-施里克近似估计折射光线
            if random() < Self::schlick(cos_theta, etai_over_etat) {
                unit_direction.reflect(&rec.normal)
            } else {
                unit_direction.refract(&rec.normal, etai_over_etat)
            }
        };
        Some(Scatter {
            attenuation: Vec3::ones(),
            ray: Ray::new(rec.p, direction),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DummyMaterial {}

impl Material for DummyMaterial {
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Option<Scatter> {
        None
    }
}
