use std::str::FromStr;
use std::sync::Arc;

use crate::camera::{Camera, PositionalCamera, SimpleCamera};
use crate::hittable::{Hittable, HittableList};
use crate::material::{Dielectric, DummyMaterial, Lambertian, Material, Metal};
use crate::math::{floats, random, Float};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

type RayColorFn = fn(&Ray, &dyn Hittable, i32) -> Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Scene {
    Weekend,
    DiffuseSpheres,
    ShinyMetal,
    FuzzyMetal,
    HollowGlassSpheres,
}

impl FromStr for Scene {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "weekend" => Ok(Scene::Weekend),
            "diffuse-spheres" => Ok(Scene::DiffuseSpheres),
            "shiny-metal" => Ok(Scene::ShinyMetal),
            "fuzzy-metal" => Ok(Scene::FuzzyMetal),
            "hollow-glass-spheres" => Ok(Scene::HollowGlassSpheres),
            _ => Err(format!("Unknown scene {}", s)),
        }
    }
}

fn background_color(y: Float) -> Vec3 {
    let t = 0.5 * (y + 1.0);
    Vec3::new(0.5, 0.7, 1.0) * t + (1. - t)
}

fn two_sphere(ground: Arc<dyn Material>, center: Arc<dyn Material>) -> HittableList {
    let hittables: Vec<Arc<dyn Hittable>> = vec![
        Arc::new(Sphere::new_xyzr(0.0, 0.0, -1.0, 0.5, center)),
        Arc::new(Sphere::new_xyzr(0.0, -100.5, -1.0, 100.0, ground)),
    ];
    HittableList::from(hittables)
}

/// 插图 10: 渲染带有半球散射的漫反射球体
pub fn diffuse_spheres(aspect_ratio: Float) -> (Arc<dyn Camera>, HittableList, RayColorFn) {
    // 场景
    let dummy_material = Arc::new(DummyMaterial {});
    let world = two_sphere(dummy_material.clone(), dummy_material);

    // 摄影机
    let viewport_height = 2.0;
    let camera = Arc::new(SimpleCamera::new(viewport_height, aspect_ratio));

    // 着色函数
    fn diffuse_spheres_ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
        if depth <= 0 {
            return Vec3::zeros();
        }
        if let Some(rec) = world.hit(ray, 0.001, floats::MAX) {
            let target = rec.p + rec.normal + Vec3::random_unit_vector();
            diffuse_spheres_ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5
        } else {
            background_color(ray.direction.unit().y)
        }
    }

    (camera, world, diffuse_spheres_ray_color)
}

/// 插图 11: 闪亮金属
pub fn shiny_metal(aspect_ratio: Float) -> (Arc<dyn Camera>, HittableList, RayColorFn) {
    let material_ground = Arc::new(Lambertian::new(0.8, 0.8, 0.0));
    let material_center = Arc::new(Lambertian::new(0.7, 0.3, 0.3));
    let material_left = Arc::new(Metal::new(0.8, 0.8, 0.8, 0.0));
    let material_right = Arc::new(Metal::new(0.8, 0.6, 0.2, 0.0));

    let hittables: Vec<Arc<dyn Hittable>> = vec![
        Arc::new(Sphere::new_xyzr(0.0, -100.5, -1.0, 100.0, material_ground)),
        Arc::new(Sphere::new_xyzr(0.0, 0.0, -1.0, 0.5, material_center)),
        Arc::new(Sphere::new_xyzr(-1.0, 0.0, -1.0, 0.5, material_left)),
        Arc::new(Sphere::new_xyzr(1.0, 0.0, -1.0, 0.5, material_right)),
    ];
    let world = HittableList::from(hittables);

    // 摄影机
    let viewport_height = 2.0;
    let camera = Arc::new(SimpleCamera::new(viewport_height, aspect_ratio));
    (camera, world, ray_color)
}

/// 插图 12: 模糊金属
pub fn fuzzy_metal(aspect_ratio: Float) -> (Arc<dyn Camera>, HittableList, RayColorFn) {
    let material_ground = Arc::new(Lambertian::new(0.8, 0.8, 0.0));
    let material_center = Arc::new(Lambertian::new(0.7, 0.3, 0.3));
    let material_left = Arc::new(Metal::new(0.8, 0.8, 0.8, 0.3));
    let material_right = Arc::new(Metal::new(0.8, 0.6, 0.2, 1.0));

    let hittables: Vec<Arc<dyn Hittable>> = vec![
        Arc::new(Sphere::new_xyzr(0.0, -100.5, -1.0, 100.0, material_ground)),
        Arc::new(Sphere::new_xyzr(0.0, 0.0, -1.0, 0.5, material_center)),
        Arc::new(Sphere::new_xyzr(-1.0, 0.0, -1.0, 0.5, material_left)),
        Arc::new(Sphere::new_xyzr(1.0, 0.0, -1.0, 0.5, material_right)),
    ];
    let world = HittableList::from(hittables);

    // 摄影机
    let viewport_height = 2.0;
    let camera = Arc::new(SimpleCamera::new(viewport_height, aspect_ratio));
    (camera, world, ray_color)
}

/// 插图 16: 中空玻璃球
pub fn hollow_glass_sphere(aspect_ratio: Float) -> (Arc<dyn Camera>, HittableList, RayColorFn) {
    // 场景
    let material_ground = Arc::new(Lambertian::new(0.8, 0.8, 0.0));
    let material_center = Arc::new(Lambertian::new(0.1, 0.2, 0.5));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(0.8, 0.6, 0.2, 0.0));
    let hittables: Vec<Arc<dyn Hittable>> = vec![
        Arc::new(Sphere::new_xyzr(0.0, -100.5, -1.0, 100.0, material_ground)),
        Arc::new(Sphere::new_xyzr(0.0, 0.0, -1.0, 0.5, material_center)),
        Arc::new(Sphere::new_xyzr(
            -1.0,
            0.0,
            -1.0,
            0.5,
            material_left.clone(),
        )),
        Arc::new(Sphere::new_xyzr(-1.0, 0.0, -1.0, -0.4, material_left)),
        Arc::new(Sphere::new_xyzr(1.0, 0.0, -1.0, 0.5, material_right)),
    ];
    let world = HittableList::from(hittables);

    // 摄影机
    let viewport_height = 2.0;
    let camera = Arc::new(SimpleCamera::new(viewport_height, aspect_ratio));
    (camera, world, ray_color)
}

// 最终场景
pub fn random_final_scene(aspect_ratio: Float) -> (Arc<dyn Camera>, HittableList, RayColorFn) {
    let pivot = Vec3::new(4., 0.2, 0.);
    let mut hittables = (-11..11)
        .into_iter()
        .map(|a| {
            (-11..11)
                .into_iter()
                .map(|b| -> Option<Arc<dyn Hittable>> {
                    let material_score = random();
                    let x = random();
                    let z = random();
                    let center = Vec3::new(a as Float + 0.9 * x, 0.2, b as Float + 0.9 * z);
                    if (center - pivot).length() > 0.9 {
                        let material: Arc<dyn Material> = if material_score < 0.8 {
                            Arc::new(Lambertian {
                                albedo: Vec3::random().powi(2),
                            })
                        } else if material_score < 0.95 {
                            Arc::new(Metal {
                                albedo: Vec3::random_between(0.5, 1.),
                                fuzz: 0.5 * random(),
                            })
                        } else {
                            Arc::new(Dielectric::new(1.5))
                        };
                        Some(Arc::new(Sphere::new(center, 0.2, material)))
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<Arc<dyn Hittable>>>()
        })
        .flatten()
        .collect::<Vec<Arc<dyn Hittable>>>();
    // 创建中心球体
    hittables.push(Arc::new(Sphere::new_xyzr(
        0.,
        1.,
        0.,
        1.,
        Arc::new(Dielectric::new(1.5)),
    )));
    hittables.push(Arc::new(Sphere::new_xyzr(
        -4.,
        1.,
        0.,
        1.,
        Arc::new(Lambertian::new(0.4, 0.2, 0.1)),
    )));
    hittables.push(Arc::new(Sphere::new_xyzr(
        4.,
        1.,
        0.,
        1.,
        Arc::new(Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.1,
        }),
    )));
    // 创建地面
    let ground_material = Arc::new(Lambertian::new(0.5, 0.5, 0.5));
    hittables.push(Arc::new(Sphere::new_xyzr(
        0.,
        -1000.,
        0.,
        1000.,
        ground_material,
    )));
    let world = HittableList::from(hittables);
    // Camera
    let vfov = 20.0;
    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let focal_dist = 10.;
    let aperture = 0.1;
    let camera = Arc::new(PositionalCamera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focal_dist,
    ));

    (camera, world, ray_color)
}

// 最终场景的着色函数
pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        Vec3::zeros()
    } else {
        if let Some(rec) = world.hit(ray, 0.001, floats::MAX) {
            if let Some(scattered) = rec.material.scatter(ray, &rec) {
                scattered.attenuation * ray_color(&scattered.ray, world, depth - 1)
            } else {
                Vec3::zeros()
            }
        } else {
            background_color(ray.direction.y)
        }
    }
}

pub fn create_scene(
    scene: Scene,
    aspect_ratio: Float,
) -> (Arc<dyn Camera>, HittableList, RayColorFn) {
    match scene {
        Scene::DiffuseSpheres => diffuse_spheres(aspect_ratio),
        Scene::ShinyMetal => shiny_metal(aspect_ratio),
        Scene::FuzzyMetal => fuzzy_metal(aspect_ratio),
        Scene::HollowGlassSpheres => hollow_glass_sphere(aspect_ratio),
        Scene::Weekend => random_final_scene(aspect_ratio),
    }
}
