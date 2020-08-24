use std::path::Path;
use stdweb::{js, unstable::TryInto};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Image {
    pub source: String,
}

impl Image {
    /// Creates a new render ctx 2d.
    pub fn new(_: f64, _: f64) -> Self {
        Image {
            source: String::default(),
        }
    }

    /// Load an image from file path. Supports BMP and PNG extensions.
    pub fn from_path<P: std::string::ToString + AsRef<Path>>(path: P) -> Result<Self, String> {
        let source = path.to_string();

        // Register image store if not registered.
        js!(
            if(!document.hasOwnProperty("image_store")) {
                document.image_store = {
                    images: {}
                };

                document.image_store.load_image = function (src) {
                    var img = new Image();

                    var d = new Promise(function (resolve, reject) {
                        img.onload = function () {
                            this.images[src] = img;
                            resolve(img);
                        }.bind(this);

                        img.onerror = function () {
                            reject("Could not load image: " + src);
                        };
                    }.bind(this));

                    img.src = src;
                    return d;
                };
            }
        );

        js!(
            document.image_store.image = function (src) {
                return (src in this.images) ? this.images[src] : null;
            };
        );

        // load the image
        js!(
            document.image_store.load_image(@{&source});
        );

        Ok(Image { source })
    }

    /// Draws a u32 slice into the image.
    pub fn draw(&mut self, _data: &[u32]) {
        // todo
    }

    /// Gets the width.
    pub fn width(&self) -> f64 {
        let width: u64 = js!(
            var image = document.image_store.image(@{&self.source});

            if(image == null) {
                return 0;
            }

            return image.width;
        )
        .try_into()
        .unwrap();
        width as f64
    }

    /// Gets the height.
    pub fn height(&self) -> f64 {
        let height: u64 = js!(
            var image = document.image_store.image(@{&self.source});

            if(image == null) {
                return 0;
            }

            return image.height;
        )
        .try_into()
        .unwrap();

        height as f64
    }
}

// todo not yet available for web
impl From<(u32, u32, Vec<u32>)> for Image {
    fn from(image: (u32, u32, Vec<u32>)) -> Self {
        Image::new(image.0.into(), image.1.into())
    }
}
