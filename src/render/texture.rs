use std::path::Path;

use glium::texture::SrgbTexture2d;

use crate::Window;

#[derive(Debug)]
pub struct TextureSettings {}

pub struct Texture2D {
    pub texture: SrgbTexture2d,
}

impl Texture2D {
    pub fn new(wnd: &Window, path: &Path) -> Texture2D {
        use std::io::Cursor;
        let image = image::load(
            Cursor::new(std::fs::read(path).unwrap()),
            image::ImageFormat::Png,
        )
        .unwrap()
        .to_rgba8();

        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let texture = Texture2D {
            texture: SrgbTexture2d::new(wnd.display(), image).unwrap(),
        };

        texture
    }
}
