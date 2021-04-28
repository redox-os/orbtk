use std::{collections::HashMap, fs::File, io::Read, path::Path};

use rusttype::Font;

#[cfg(all(
    not(target_os = "redox"),
    not(target_arch = "wasm32"),
    feature = "floader"
))]
pub use font_loader::system_fonts;
#[cfg(all(
    not(target_os = "redox"),
    not(target_arch = "wasm32"),
    feature = "floader"
))]
pub use font_loader::system_fonts::FontProperty;
#[cfg(all(
    not(target_os = "redox"),
    not(target_arch = "wasm32"),
    feature = "floader"
))]
pub use font_loader::system_fonts::FontPropertyBuilder;

use crate::error::Error;

#[derive(Debug, Default, Clone)]
pub struct FontLoader {
    fonts: HashMap<String, Font<'static>>,
}

impl FontLoader {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }

    pub fn load_font_from_bytes(
        &mut self,
        name: impl Into<String>,
        bytes: &'static [u8],
    ) -> Result<(), Error> {
        if let Some(font) = Font::try_from_bytes(bytes) {
            self.fonts.insert(name.into(), font);
            return Ok(());
        }

        Err(Error::CannotLoadFont)
    }

    pub fn load_font_from_path<P: AsRef<Path>>(
        &mut self,
        name: impl Into<String>,
        path: P,
    ) -> Result<(), Error> {
        let mut file = File::open(path).map_err(|_| Error::CannotLoadFont)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .map_err(|_| Error::CannotLoadFont)?;

        if let Some(font) = Font::try_from_vec(data) {
            self.fonts.insert(name.into(), font);
            return Ok(());
        }

        Err(Error::CannotLoadFont)
    }

    // A function to automate the process of building a font property  from "typeface, family, style"
    #[cfg(all(
        not(target_os = "redox"),
        not(target_arch = "wasm32"),
        feature = "floader"
    ))]
    fn build_fontproperty(
        typeface: Option<&str>,
        family: Option<&str>,
        style: Option<&str>,
    ) -> FontProperty {
        let mut font = FontPropertyBuilder::new();
        if let Some(style) = style {
            let style_caps = &style.to_uppercase();
            let italic = style_caps.contains("ITALIC");
            let oblique = style_caps.contains("OBLIQUE");
            let bold = style_caps.contains("BOLD");
            if italic {
                font = font.italic();
            }
            if oblique {
                font = font.oblique();
            }
            if bold {
                font = font.bold();
            }
        }
        if let Some(typeface) = typeface {
            // FontProperty has no support for differentiating Sans and Serif.
            let typeface_caps = &typeface.to_uppercase();
            if typeface_caps.contains("MONO") {
                font = font.monospace();
            }
        }
        if let Some(family) = family {
            if let Some(typeface) = typeface {
                let typeface_caps = &typeface.to_uppercase();
                // manually adding Serif and Sans
                if typeface_caps.contains("SERIF") {
                    font = font.family(&[family, "Serif"].concat());
                } else if typeface_caps.contains("SANS") {
                    font = font.family(&[family, "Sans"].concat())
                }
            } else {
                font = font.family(family);
            }
        }
        font.build()
    }

    #[cfg(target_os = "redox")]
    pub fn load_system_font(
        &mut self,
        name: impl Into<String>,
        typeface: Option<&str>,
        family: Option<&str>,
        style: Option<&str>,
    ) -> Result<(), Error> {
        self.load_font_from_path(
            name,
            &format!(
                "/ui/fonts/{}/{}/{}.ttf",
                typeface.unwrap_or("Mono"),
                family.unwrap_or("Fira"),
                style.unwrap_or("Regular")
            ),
        )
    }

    #[cfg(all(
        not(target_os = "redox"),
        not(target_arch = "wasm32"),
        feature = "floader"
    ))]
    pub fn load_system_font(
        &mut self,
        name: impl Into<String>,
        typeface: Option<&str>,
        family: Option<&str>,
        style: Option<&str>,
    ) -> Result<(), Error> {
        // This funciton attempts to use the rust-font-loader library, a frontend
        // to the ubiquitous C library fontconfig, to find and load the specified
        // font.
        let mut font = FontLoader::build_fontproperty(typeface, family, style);
        // font_loader::query specific returns an empty vector if there are no matches
        // and does not tag the result with associated data like "italic", merely returns
        // the name of the font if it exists.
        let fonts = system_fonts::query_specific(&mut font); // Returns an empty vector if there are no matches.
                                                             // Confirm that a font matched:
        if !fonts.is_empty() {
            // get the matched font straight from the data:
            let font_data = system_fonts::get(&font); // Getting font data from properties
            match font_data {
                Some((data, _)) => {
                    if let Some(font) = Font::try_from_vec(data) {
                        self.fonts.insert(name.into(), font);
                        return Ok(());
                    }
                }
                None => return Err(Error::CannotLoadFont),
            }
        } else {
            // If no font matched, try again with no family, as concatenating "Sans" or "Serif" may rule out legitimate fonts
            let mut font = FontLoader::build_fontproperty(None, family, style);
            let fonts = system_fonts::query_specific(&mut font);
            if !fonts.is_empty() {
                let font_data = system_fonts::get(&font);
                match font_data {
                    Some((data, _)) => {
                        if let Some(font) = Font::try_from_vec(data) {
                            self.fonts.insert(name.into(), font);
                            return Ok(());
                        }
                    }
                    _ => return Err(Error::CannotLoadFont),
                }
            } else {
                // If no font matched, try to load the default font manually
                return self.load_font_from_path(
                    name,
                    "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf",
                );
            }
        }

        Ok(())
    }

    pub fn font(&self, family: &str) -> Option<Font<'static>> {
        self.fonts.get(family).cloned()
    }
}
