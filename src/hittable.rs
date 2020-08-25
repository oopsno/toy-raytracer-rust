use crate::material::AnyMaterial;
use crate::ray::Ray;
use crate::vec3::{Float, Vec3};

use std::sync::Arc;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: Float,
    pub front_face: bool,
    pub material: AnyMaterial,
}

impl HitRecord {
    pub fn create<F>(ray: &Ray, t: Float, material: AnyMaterial, f: F) -> HitRecord
    where
        F: Fn(Vec3) -> Vec3,
    {
        let p = ray.at(t);
        let outward_normal = f(p);
        let front_face = ray.direction.dot(&outward_normal) < 0.;
        HitRecord {
            p,
            t,
            front_face,
            normal: if front_face {
                outward_normal.unit()
            } else {
                -outward_normal.unit()
            },
            material,
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub struct HittableList {
    hittables: Vec<Arc<dyn Hittable>>,
}

impl From<Vec<Arc<dyn Hittable>>> for HittableList {
    fn from(hittables: Vec<Arc<dyn Hittable>>) -> Self {
        HittableList { hittables }
    }
}

unsafe impl Sync for HittableList {}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let mut farthest = t_max;
        let mut nearst: Option<HitRecord> = None;
        for obj in self.hittables.iter() {
            if let Some(rec) = obj.hit(ray, t_min, farthest) {
                farthest = rec.t;
                nearst = Some(rec);
            };
        }
        nearst
    }
}
