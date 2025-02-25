use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::Vector3;
use crate::hittable::HitRecord;
use rand::Rng;

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0_squared = r0 * r0;
        r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
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

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let random_double = rand::thread_rng().gen_range(0.0..1.0);
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double {
            Vector3::reflect(&unit_direction, &hit_record.normal)
        } else {
            Vector3::refract(&unit_direction, &hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit_record.point, direction);
        Some((attenuation, scattered))
    }
}