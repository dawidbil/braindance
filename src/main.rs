use std::fs::File;
use std::io::{Write, BufWriter};

mod vector3;
use vector3::{Point3, Vector3};

mod color;
use color::Color;

mod viewport;
use viewport::Viewport;

mod ray;
use ray::Ray;

mod utils;
use utils::lerp;

const IMAGE_WIDTH: u32 = 256;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;
const CAMERA_CENTER_TUPLE: (f64, f64, f64) = (0.0, 0.0, 0.0);

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), t)
}

fn render(viewport: &Viewport, writer: &mut BufWriter<File>) -> Result<(), std::io::Error> {
    write!(writer, "P3\n{} {}\n255\n", viewport.image_width, viewport.image_height)?;

    for j in 0..viewport.image_height {
        println!("\rScanlines remaining: {}", viewport.image_height - j);
        for i in 0..viewport.image_width {
            let pixel_center = viewport.pixel_center(i, j);
            let ray_direction = pixel_center.sub(&viewport.camera_center);
            let ray = Ray::new(viewport.camera_center, ray_direction);
            let color = ray_color(&ray);
            color.dump(writer)?;
        }
    }
    println!("\rDone");
    Ok(())
}

fn main() {
    let camera_center = Point3::new(
        CAMERA_CENTER_TUPLE.0,
        CAMERA_CENTER_TUPLE.1,
        CAMERA_CENTER_TUPLE.2,
    );
    let viewport = Viewport::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        VIEWPORT_HEIGHT,
        FOCAL_LENGTH,
        camera_center,
    );
    let file = File::create("image.ppm").unwrap();
    let mut writer = BufWriter::new(file);
    render(&viewport, &mut writer).expect("Failed to dump image");
    println!("Viewport: {:#?}", viewport);
}
