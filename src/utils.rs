use crate::color::Color;

#[macro_export]
macro_rules! assert_near_eq {
    ($a:expr, $b:expr) => {
        assert!(($a - $b).abs() < 1e-10, "Expected {} to be near {}", $a, $b);
    };
    ($a:expr, $b:expr, $epsilon:expr) => {
        assert!(
            ($a - $b).abs() < $epsilon,
            "Expected {} to be near {} (within {})",
            $a,
            $b,
            $epsilon
        );
    };
}

pub fn lerp(a: Color, b: Color, t: f64) -> Color {
    Color::new(
        a.r() * (1.0 - t) + b.r() * t,
        a.g() * (1.0 - t) + b.g() * t,
        a.b() * (1.0 - t) + b.b() * t,
    )
}
