use stdweb::{js, unstable::TryInto};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Image {
    pub source: String,
}

impl Image {
    /// Constructs a new image with the given source.
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();

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

        Image { source }
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
