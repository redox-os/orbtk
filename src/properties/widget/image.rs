
pub use orbimage::Image as OrbImage;
pub use orbclient::Renderer;
use orbclient::Color;

pub struct Image(OrbImage);

property!(Image, ImageProperty, image, shared_image);

impl Image {
    pub fn width(&self) -> u32 {
        self.0.width()
    }

    pub fn height(&self) -> u32 {
        self.0.height()
    }

    pub fn data(&self) -> &[Color] {
        self.0.data()
    }
}

impl From<&str> for Image {
    fn from(s: &str) -> Image {
        Image(OrbImage::from_path(s).unwrap())
    }
}

impl From<String> for Image {
    fn from(s: String) -> Image {
        Image(OrbImage::from_path(s).unwrap())
    }
}




