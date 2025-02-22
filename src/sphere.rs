use crate::assert_near_eq;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vector3::Point3;
use crate::material::Material;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Box<dyn Material>) -> Self {
        if radius <= 0.0 {
            panic!("Radius must be positive");
        }
        Sphere { center, radius, material }
    }

    fn is_t_valid(&self, t: f64, ray_tmin: f64, ray_tmax: f64) -> bool {
        t > ray_tmin && t < ray_tmax
    }

    fn get_hit_record(&self, ray: &Ray, t: f64) -> HitRecord {
        let point = ray.at(t);
        assert_near_eq!(point.sub(&self.center).length(), self.radius);
        let outward_normal = point.sub(&self.center).div(self.radius);
        HitRecord::new(point, outward_normal, t, ray.direction, &*self.material)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center.sub(&ray.origin);
        let a = ray.direction.dot(&ray.direction);
        let h = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if self.is_t_valid(root, ray_tmin, ray_tmax) {
            return Some(self.get_hit_record(ray, root));
        }

        root = (h + sqrtd) / a;
        if self.is_t_valid(root, ray_tmin, ray_tmax) {
            return Some(self.get_hit_record(ray, root));
        }

        None
    }
}
