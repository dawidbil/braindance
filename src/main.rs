#![allow(dead_code)]

use std::fs::File;
use std::io::BufWriter;

mod vector3;
use vector3::Point3;

mod color;
mod camera;
use camera::Camera;
mod ray;
mod utils;
mod sphere;
use sphere::Sphere;
mod hittables;
use hittables::Hittables;
mod hittable;

const IMAGE_WIDTH: u32 = 1600;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;
const CAMERA_CENTER_TUPLE: (f64, f64, f64) = (0.0, 0.0, 0.0);

fn main() {
    let camera_center = Point3::new(
        CAMERA_CENTER_TUPLE.0,
        CAMERA_CENTER_TUPLE.1,
        CAMERA_CENTER_TUPLE.2,
    );
    let camera = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        VIEWPORT_HEIGHT,
        FOCAL_LENGTH,
        camera_center,
    );
    // println!("Camera: {:#?}", camera);
    let mut hittables = Hittables::new();
    hittables.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    hittables.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    let file = File::create("image.ppm").unwrap();
    let mut writer = BufWriter::new(file);
    camera.render(&mut writer, &hittables).expect("Failed to dump image");
}
