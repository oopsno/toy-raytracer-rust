use crate::math::Float;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Camera: Sync + Send {
    fn ray(&self, u: Float, v: Float) -> Ray;
}

pub struct SimpleCamera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left: Vec3,
}

impl SimpleCamera {
    pub fn new(viewpoint_height: Float, aspect_ratio: Float) -> Self {
        let viewpoint_width = viewpoint_height * aspect_ratio;
        let focal_length = 1.0;
        let origin = Vec3::zeros();
        let horizontal = Vec3::new(viewpoint_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewpoint_height, 0.0);
        Self {
            origin,
            horizontal,
            vertical,
            lower_left: (origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::new(0.0, 0.0, focal_length)),
        }
    }
}

unsafe impl Sync for SimpleCamera {}

impl Camera for SimpleCamera {
    fn ray(&self, u: Float, v: Float) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + self.horizontal * u + self.vertical * v - self.origin,
            t: 0.0,
        }
    }
}

pub struct PositionalCamera {
    pub viewpoint_height: Float,
    pub viewpoint_width: Float,
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left: Vec3,
    pub lens_radius: Float,
    u: Vec3,
    v: Vec3,
}

unsafe impl Sync for PositionalCamera {}

impl PositionalCamera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: Float,
        aspect_ratio: Float,
        aperture: Float,
        focal_dist: Float,
    ) -> PositionalCamera {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewpoint_height = 2.0 * h;
        let viewpoint_width = aspect_ratio * viewpoint_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * (viewpoint_width * focal_dist);
        let vertical = v * (viewpoint_height * focal_dist);
        let lower_left = origin - horizontal / 2. - vertical / 2. - focal_dist * w;

        PositionalCamera {
            viewpoint_width,
            viewpoint_height,
            origin,
            horizontal,
            vertical,
            lower_left,
            lens_radius: aperture / 2.,
            u,
            v,
        }
    }
}

impl Camera for PositionalCamera {
    fn ray(&self, s: Float, t: Float) -> Ray {
        let rd = Vec3::random_in_unit_sphere() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
            t: 0.0,
        }
    }
}
