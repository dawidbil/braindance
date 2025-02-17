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
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    /// Create a new color with the given red, green, and blue values.
    /// All values must be between 0.0 and 1.0.
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        check_color_value(r);
        check_color_value(g);
        check_color_value(b);
        Color { r, g, b }
    }

    pub fn from_vec(v: Vector3) -> Self {
        Self::new(v.x, v.y, v.z)
    }

    /// Dump the color to a writer.
    /// Scales the color to the range 0-255 and writes it to the writer.
    pub fn dump<T: Write>(&self, out: &mut T) -> Result<(), std::io::Error> {
        write!(out, "{} {} {}\n", to_256(self.r), to_256(self.g), to_256(self.b))
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color({} {} {})", self.r, self.g, self.b)?;
        write!(f, "Color({} {} {})", to_256(self.r), to_256(self.g), to_256(self.b))?;
        Ok(())
    }
}

