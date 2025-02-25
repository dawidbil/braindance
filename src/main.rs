#![allow(dead_code)]

use std::{f64::consts::PI, fs::File, io::BufWriter};

mod camera;
mod color;
mod hittable;
mod hittables;
mod ray;
mod sphere;
mod utils;
mod vector3;
mod material;
mod lambertian;
mod metal;
mod dielectric;

use camera::Camera;
use hittables::Hittables;
use sphere::Sphere;
use vector3::Point3;
use lambertian::Lambertian;
use metal::Metal;
use color::Color;
use dielectric::Dielectric;

const IMAGE_WIDTH: u32 = 512;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const FOCAL_LENGTH: f64 = 1.0;
const VFOW: f64 = 90.0;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;
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
        FOCAL_LENGTH,
        VFOW,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        camera_center,
    );
    // println!("Camera: {:#?}", camera);
    let radius= (PI / 4.0).cos();
    let material_left = Box::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Box::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    let mut hittables = Hittables::new();
    hittables.add(Box::new(Sphere::new(Point3::new(-radius, 0.0, -1.0), radius, material_left)));
    hittables.add(Box::new(Sphere::new(Point3::new(radius, 0.0, -1.0), radius, material_right)));

    let file = File::create("image.ppm").unwrap();
    let mut writer = BufWriter::new(file);
    camera
        .render(&mut writer, &hittables)
        .expect("Failed to dump image");
}
