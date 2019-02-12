use orbclient::Color;
pub use orbclient::Renderer;
pub use orbimage::Image as OrbImage;

#[derive(Clone)]
pub struct Image(pub OrbImage);

impl Default for Image {
    fn default() -> Self {
        Image(OrbImage::new(0, 0))
    }
}

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
