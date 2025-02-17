use crate::color::Color;

pub fn lerp(a: Color, b: Color, t: f64) -> Color {
    Color::new(
        a.r() * (1.0 - t) + b.r() * t,
        a.g() * (1.0 - t) + b.g() * t,
        a.b() * (1.0 - t) + b.b() * t,
    )
}

