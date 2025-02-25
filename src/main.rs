#![allow(dead_code)]

use std::fs::File;
use std::io::BufWriter;
use rand::Rng;

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
use vector3::{Point3, Vector3};
use lambertian::Lambertian;
use metal::Metal;
use color::Color;
use dielectric::Dielectric;

const IMAGE_WIDTH: u32 = 1200;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VFOV: f64 = 20.0;
const SAMPLES_PER_PIXEL: u32 = 20;
const MAX_DEPTH: u32 = 50;
const DEFOCUS_ANGLE: f64 = 0.6;
const FOCUS_DISTANCE: f64 = 10.0;

fn main() {
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        VFOV,
        lookfrom,
        lookat,
        vup,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        DEFOCUS_ANGLE,
        FOCUS_DISTANCE,
    );
    let mut hittables = Hittables::new();

    let ground_material = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    hittables.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::thread_rng().gen_range(0.0..1.0);
            let center = Point3::new(a as f64 + 0.9 * rand::thread_rng().gen_range(0.0..1.0), 0.2, b as f64 + 0.9 * rand::thread_rng().gen_range(0.0..1.0));
            if center.sub(&Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vector3::random(0.0, 1.0).to_color();
                    let sphere_material = Box::new(Lambertian::new(albedo));
                    hittables.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Vector3::random(0.5, 1.0).to_color();
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    let sphere_material = Box::new(Metal::new(albedo, fuzz));
                    hittables.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Box::new(Dielectric::new(1.5));
                    hittables.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Box::new(Dielectric::new(1.5));
    hittables.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    hittables.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    hittables.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let file = File::create("image.ppm").unwrap();
    let mut writer = BufWriter::new(file);
    camera
        .render(&mut writer, &hittables)
        .expect("Failed to dump image");
    println!("Camera: {:#?}", camera);
}
