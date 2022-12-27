/// RGBA representation of colors. Each value goes from 0 to 1.
///
/// You can use the constants for some default values.
///
/// #Examples
/// ``` rust
/// use ldrawy::Color;
/// let white_color = Color::WHITE;
/// let white_color = Color::new(1.0, 1.0, 1.0, 1.0);
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self { Self { r, g, b, a } }

    pub const CLEAR: Color = Color::new(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    pub const GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
    pub const SILVER: Color = Color::new(0.75, 0.75, 0.75, 1.0);
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    pub const MAROON: Color = Color::new(0.5, 0.0, 0.0, 1.0);
    pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
    pub const PURPLE: Color = Color::new(0.5, 0.0, 0.5, 1.0);
    pub const GREEN: Color = Color::new(0.0, 0.5, 0.0, 1.0);
    pub const LIME: Color = Color::new(0.0, 1.0, 0.0, 1.0);
    pub const YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
    pub const BLUE_NAVY: Color = Color::new(0.0, 0.0, 0.5, 1.0);
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
    pub const BLUE_TEAL: Color = Color::new(0.0, 0.5, 0.5, 1.0);
    pub const BLUE_AQUA: Color = Color::new(0.0, 1.0, 1.0, 1.0);
}

impl Into<wgpu::Color> for Color {
    fn into(self) -> wgpu::Color {
        wgpu::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}
