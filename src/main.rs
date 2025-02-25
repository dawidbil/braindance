#![allow(dead_code)]

use std::{fs::File, io::BufWriter};

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
const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;
const SAMPLES_PER_PIXEL: u32 = 50;
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
        VIEWPORT_HEIGHT,
        FOCAL_LENGTH,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        camera_center,
    );
    // println!("Camera: {:#?}", camera);
    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Box::new(Dielectric::new(1.0 / 1.33));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let mut hittables = Hittables::new();
    hittables.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    hittables.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    hittables.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    hittables.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));
    let file = File::create("image.ppm").unwrap();
    let mut writer = BufWriter::new(file);
    camera
        .render(&mut writer, &hittables)
        .expect("Failed to dump image");
}
