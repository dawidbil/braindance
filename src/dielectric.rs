use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::Vector3;
use crate::hittable::HitRecord;

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self { refraction_index: index_of_refraction }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction.normalize();
        let cos_theta = unit_direction.neg().dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0 {
            Vector3::reflect(&unit_direction, &hit_record.normal)
        } else {
            Vector3::refract(&unit_direction, &hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit_record.point, direction);
        Some((attenuation, scattered))
    }
}