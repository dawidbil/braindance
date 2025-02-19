use std::fs::File;
use std::io::{BufWriter, Write};

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
    pub focal_length: f64,
    pub camera_center: Point3,
    pub viewport_u: Vector3,
    pub viewport_v: Vector3,
    pub pixel_delta_u: Vector3,
    pub pixel_delta_v: Vector3,
    pub viewport_upper_left: Point3,
    pub pixel_upper_left: Point3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        viewport_height: f64,
        focal_length: f64,
        camera_center: Point3,
    ) -> Self {
        let image_height = match (image_width as f64 / aspect_ratio) as u32 {
            0 => 1,
            height => height,
        };
        let viewport_aspect_ratio = image_width as f64 / image_height as f64;
        let viewport_width = viewport_height * viewport_aspect_ratio;
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u.div(image_width as f64);
        let pixel_delta_v = viewport_v.div(image_height as f64);
        let viewport_upper_left = camera_center
            .sub(&Vector3::new(0.0, 0.0, focal_length))
            .sub(&viewport_u.div(2.0))
            .sub(&viewport_v.div(2.0));
        let pixel_upper_left = viewport_upper_left.add(&pixel_delta_u.add(&pixel_delta_v).mul(0.5));
        Self {
            aspect_ratio,
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            focal_length,
            camera_center,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel_upper_left,
        }
    }

    fn pixel_center(&self, i: u32, j: u32) -> Point3 {
        let pixel_delta_u_i = self.pixel_delta_u.mul(i as f64);
        let pixel_delta_v_j = self.pixel_delta_v.mul(j as f64);
        self.pixel_upper_left
            .add(&pixel_delta_u_i)
            .add(&pixel_delta_v_j)
    }

    fn ray_color(&self, ray: &Ray, hittables: &Hittables) -> Color {
        if let Some(hit) = hittables.hit(ray, 0.0, f64::INFINITY) {
            let normal = hit.normal;
            return Vector3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0)
                .mul(0.5)
                .to_color();
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
                let pixel_center = self.pixel_center(i, j);
                let ray_direction = pixel_center.sub(&self.camera_center);
                let ray = Ray::new(self.camera_center, ray_direction);
                let color = self.ray_color(&ray, &hittables);
                color.dump(writer)?;
            }
        }
        println!("\r{:-^30}", "Done");
        Ok(())
    }
}
