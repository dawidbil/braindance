use rand::Rng;
use crate::color::Color;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn neg(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn add(&self, other: &Vector3) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: &Vector3) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn mul(&self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn div(&self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        self.div(length)
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(min..max);
        let y = rng.gen_range(min..max);
        let z = rng.gen_range(min..max);
        Self::new(x, y, z)
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random(-1.0, 1.0);
            let length = p.length();
            if length <= 1e-160 {
                continue;
            }
            if length <= 1.0 {
                return p.div(length);
            }
        }
    }

    pub fn random_in_hemisphere(normal: &Vector3) -> Self {
        let in_unit_sphere = Self::random_unit_vector();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            in_unit_sphere.neg()
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(vector: &Vector3, normal: &Vector3) -> Self {
        vector.sub(&normal.mul(2.0 * vector.dot(normal)))
    }

    pub fn to_color(self) -> Color {
        Color::new(self.x, self.y, self.z)
    }
}

pub type Point3 = Vector3;
