use crate::ray::Ray;
use crate::vector3::{Point3, Vector3};
use crate::material::Material;

pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(point: Point3, outward_normal: Vector3, t: f64, ray_direction: Vector3, material: &'a dyn Material) -> Self {
        let front_face = ray_direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            outward_normal.neg()
        };
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
