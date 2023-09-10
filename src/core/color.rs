use bytemuck::{Pod, Zeroable};

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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    ///Should always return 16 bytes.
    pub const fn size_in_bytes() -> u32 { std::mem::size_of::<Self>() as u32 }
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self { Self { r, g, b, a } }
    // pub const fn from_bytes(bytes: [u8; 16]) -> Self {
    //     Self {
    //         r: f32::from_be_bytes(<[u8; 4]>::try_from(bytes[0..3])),
    //         g: f32::from_be_bytes(bytes[4..7].try_into().unwrap()),
    //         b: f32::from_be_bytes(bytes[8..11].try_into().unwrap()),
    //         a: f32::from_be_bytes(bytes[12..15].try_into().unwrap()),
    //     }
    // }

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

    pub fn to_rgba8(&self) -> [u8; 4] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        ]
    }
}

impl From<Color> for wgpu::Color {
    fn from(val: Color) -> Self {
        wgpu::Color {
            r: val.r as f64,
            g: val.g as f64,
            b: val.b as f64,
            a: val.a as f64,
        }
    }
}

impl From<egui::Color32> for Color {
    fn from(value: egui::epaint::Color32) -> Self {
        Color {
            r: value.r() as f32 / 255.,
            g: value.g() as f32 / 255.,
            b: value.b() as f32 / 255.,
            a: value.a() as f32 / 255.,
        }
    }
}
