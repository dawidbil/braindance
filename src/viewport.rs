use crate::vector3::{Point3, Vector3};

#[derive(Debug)]
pub struct Viewport {
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

impl Viewport {
    pub fn new(aspect_ratio: f64, image_width: u32, viewport_height: f64, focal_length: f64, camera_center: Point3) -> Self {
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
        let viewport_upper_left = camera_center.sub(&Vector3::new(0.0, 0.0, focal_length)).sub(&viewport_u.div(2.0)).sub(&viewport_v.div(2.0));
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

    pub fn pixel_center(&self, i: u32, j: u32) -> Point3 {
        let pixel_delta_u_i = self.pixel_delta_u.mul(i as f64);
        let pixel_delta_v_j = self.pixel_delta_v.mul(j as f64);
        self.pixel_upper_left.add(&pixel_delta_u_i).add(&pixel_delta_v_j)
    }
}
