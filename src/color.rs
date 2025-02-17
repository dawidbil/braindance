use std::io::Write;
use crate::vector3::Vector3;
use std::fmt::{self, Display};

fn to_256(x: f64) -> u8 {
    (x * 255.0) as u8
}

fn check_color_value(x: f64) {
    if x < 0.0 || x > 1.0 {
        panic!("Invalid color value: {}", x);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    vec: Vector3
}

impl Color {
    /// Create a new color with the given red, green, and blue values.
    /// All values must be between 0.0 and 1.0.
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        check_color_value(r);
        check_color_value(g);
        check_color_value(b);
        Color { vec: Vector3::new(r, g, b) }
    }

    pub fn r(&self) -> f64 { self.vec.x }
    pub fn g(&self) -> f64 { self.vec.y }
    pub fn b(&self) -> f64 { self.vec.z }

    pub fn add(&self, other: &Color) -> Self {
        Self { vec: self.vec.add(&other.vec) }
    }

    pub fn sub(&self, other: &Color) -> Self {
        Self { vec: self.vec.sub(&other.vec) }
    }

    pub fn mul(&self, scalar: f64) -> Self {
        Self { vec: self.vec.mul(scalar) }
    }

    pub fn div(&self, scalar: f64) -> Self {
        Self { vec: self.vec.div(scalar) }
    }

    /// Convert from Vector3
    pub fn from_vec(v: Vector3) -> Self {
        Self::new(v.x, v.y, v.z)
    }

    /// Convert to Vector3
    pub fn as_vec(&self) -> &Vector3 {
        &self.vec
    }

    /// Dump the color to a writer.
    /// Scales the color to the range 0-255 and writes it to the writer.
    pub fn dump<T: Write>(&self, out: &mut T) -> Result<(), std::io::Error> {
        write!(out, "{} {} {}\n", 
            to_256(self.r()), 
            to_256(self.g()), 
            to_256(self.b()))
    }
}

// Implement From for clean conversions
impl From<Vector3> for Color {
    fn from(vec: Vector3) -> Self {
        Self::from_vec(vec)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color({} {} {})", self.r(), self.g(), self.b())?;
        write!(f, " RGB({} {} {})", 
            to_256(self.r()), 
            to_256(self.g()), 
            to_256(self.b()))?;
        Ok(())
    }
}

