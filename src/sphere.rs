use crate::hittable::{HitRecord, Hittable};
use crate::material::AnyMaterial;
use crate::ray::Ray;
use crate::vec3::{Float, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: Float,
    pub material: AnyMaterial,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float, material: AnyMaterial) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn new_xyzr(x: Float, y: Float, z: Float, r: Float, material: AnyMaterial) -> Sphere {
        Sphere {
            center: Vec3::new(x, y, z),
            radius: r,
            material,
        }
    }
}

unsafe impl Sync for Sphere {}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b_2 = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let d = b_2.powi(2) - a * c;
        if d >= 0. {
            // d == 0 是小概率事件
            let d_sqrt = d.sqrt();
            vec![(-b_2 - d_sqrt) / a, (-b_2 + d_sqrt) / a]
                .into_iter()
                .find(|t| t_min < *t && *t < t_max)
                .map(|t| {
                    HitRecord::create(ray, t, self.material.clone(), |p| {
                        ((p - self.center) / self.radius).unit()
                    })
                })
        } else {
            None
        }
    }
}
