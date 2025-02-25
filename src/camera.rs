use std::fs::File;
use std::io::{BufWriter, Write};
use rand::Rng;
use crate::color::Color;
use crate::hittables::Hittables;
use crate::ray::Ray;
use crate::utils::lerp;
use crate::vector3::{Point3, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vector3,
    pub camera_u: Vector3,
    pub camera_v: Vector3,
    pub camera_w: Vector3,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub viewport_u: Vector3,
    pub viewport_v: Vector3,
    pub pixel_delta_u: Vector3,
    pub pixel_delta_v: Vector3,
    pub viewport_upper_left: Point3,
    pub pixel_upper_left: Point3,
    pub defocus_angle: f64,
    pub focus_distance: f64,
    pub defocus_disk_u: Vector3,
    pub defocus_disk_v: Vector3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vector3,
        samples_per_pixel: u32,
        max_depth: u32,
        defocus_angle: f64,
        focus_distance: f64,
    ) -> Self {
        let image_height = match (image_width as f64 / aspect_ratio) as u32 {
            0 => 1,
            height => height,
        };
        let viewport_aspect_ratio = image_width as f64 / image_height as f64;
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * viewport_aspect_ratio;
        let camera_w = lookfrom.sub(&lookat).normalize();
        let camera_u = vup.cross(&camera_w).normalize();
        let camera_v = camera_w.cross(&camera_u);
        let viewport_u = camera_u.mul(viewport_width);
        let viewport_v = camera_v.neg().mul(viewport_height);
        let pixel_delta_u = viewport_u.div(image_width as f64);
        let pixel_delta_v = viewport_v.div(image_height as f64);
        let viewport_upper_left = lookfrom
            .sub(&viewport_u.div(2.0))
            .sub(&viewport_v.div(2.0))
            .sub(&camera_w.mul(focus_distance));
        let pixel_upper_left = viewport_upper_left.add(&pixel_delta_u.add(&pixel_delta_v).mul(0.5));
        let defocus_radius = focus_distance * (defocus_angle.to_radians() / 2.0).tan();
        let defocus_disk_u = camera_u.mul(defocus_radius);
        let defocus_disk_v = camera_v.mul(defocus_radius);
        Self {
            aspect_ratio,
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            vfov,
            lookfrom,
            lookat,
            vup,
            camera_u,
            camera_v,
            camera_w,
            samples_per_pixel,
            max_depth,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel_upper_left,
            defocus_angle,
            focus_distance,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel_upper_left
            .add(&self.pixel_delta_u.mul(i as f64 + offset.x))
            .add(&self.pixel_delta_v.mul(j as f64 + offset.y));
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.lookfrom
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample.sub(&ray_origin);
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vector3 {
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();
        Vector3::new(px, py, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vector3 {
        let p = Vector3::random_in_unit_disk();
        self.defocus_disk_u.mul(p.x).add(&self.defocus_disk_v.mul(p.y)).add(&self.lookfrom)
    }

    fn ray_color(&self, ray: &Ray, hittables: &Hittables, depth: u32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = hittables.hit(ray, 0.001, f64::INFINITY) {
            if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit) {
                let color = self.ray_color(&scattered, hittables, depth - 1);
                return Color::new(
                    attenuation.r() * color.r(),
                    attenuation.g() * color.g(),
                    attenuation.b() * color.b(),
                );
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), t)
    }

    pub fn render(
        &self,
        writer: &mut BufWriter<File>,
        hittables: &Hittables,
    ) -> Result<(), std::io::Error> {
        write!(
            writer,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )?;

        for j in 0..self.image_height {
            print!("\rScanlines remaining: {:>4}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color = pixel_color.add(&self.ray_color(&ray, &hittables, self.max_depth));
                }
                pixel_color = pixel_color.div(self.samples_per_pixel as f64);
                pixel_color.dump(writer)?;
            }
        }
        println!("\r{:-^30}", "Done");
        Ok(())
    }
}
