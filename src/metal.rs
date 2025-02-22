use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::Vector3;
use crate::hittable::HitRecord;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        if fuzz < 0.0 || fuzz > 1.0 {
            panic!("Fuzz must be between 0.0 and 1.0");
        }
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = Vector3::reflect(&ray_in.direction, &hit_record.normal);
        reflected = reflected.normalize().add(&Vector3::random_unit_vector().mul(self.fuzz));
        let scattered = Ray::new(hit_record.point, reflected);
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
