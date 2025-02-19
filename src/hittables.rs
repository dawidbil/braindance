use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct Hittables {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Hittables {
    pub fn new() -> Self {
        Hittables {
            hittables: Vec::new(),
        }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable);
    }

    pub fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = ray_tmax;

        for hittable in self.hittables.iter() {
            if let Some(record) = hittable.hit(ray, ray_tmin, closest_so_far) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }
}
