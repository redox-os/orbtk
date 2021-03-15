include!(concat!(env!("OUT_DIR"), "/colors.rs"));

#[cfg(not(feature = "no_std"))]
use std::fmt;

/// A r g b a color.
#[derive(Copy, Clone, PartialOrd, Default)]
#[repr(packed)]
pub struct Color {
    pub data: u32,
}

impl Color {
    /// Create a new color from RGB
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            data: 0xFF00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    /// Create a new color from HSV(0.0-360.0, 0.0-1.0, 0.0-1.0)
    pub fn hsv(h: f64, s: f64, v: f64) -> Self {
        Self::hsva(h, s, v, 1.0)
    }

    /// Create a new color from HSL(0.0-360.0, 0.0-1.0, 0.0-1.0)
    pub fn hsl(h: f64, s: f64, l: f64) -> Self {
        Self::hsla(h, s, l, 1.0)
    }

    /// Create a new color from RGB and alpha values
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            data: ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    /// Create a new color from HSV(0.0-360.0, 0.0-1.0, 0.0-1.0) and alpha values(0.0-1.0)
    pub fn hsva(mut hue: f64, mut saturation: f64, mut value: f64, alpha: f64) -> Self {
        hue %= 360.0;
        saturation = saturation.max(0.0).min(1.0);
        value = value.max(0.0).min(1.0);
        let hh = hue / 60.0;
        let idx = hh.floor() as i32;
        let ff = hh.fract();
        let chroma = value * (1.0 - saturation);
        let second_component = value * (1.0 - (saturation * ff));
        let t = value * (1.0 - (saturation * (1.0 - ff)));
        let (r, g, b) = match idx {
            0 => (value, t, chroma),
            1 => (second_component, value, chroma),
            2 => (chroma, value, t),
            3 => (chroma, second_component, value),
            4 => (t, chroma, value),
            5 => (value, chroma, second_component),
            _ => unreachable!(),
        };
        Self::rgba(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            (alpha * 255.0) as u8,
        )
    }

    /// Create a new color from HSL(0.0-360.0, 0.0-1.0, 0.0-1.0) and alpha values(0.0-1.0)
    pub fn hsla(mut hue: f64, mut saturation: f64, mut lightness: f64, alpha: f64) -> Self {
        hue %= 360.0;
        saturation = saturation.max(0.0).min(1.0);
        lightness = lightness.max(0.0).min(1.0);
        let hh = hue / 60.0;
        let idx = hh.floor() as i32;
        let chroma = (1.0 - ((2.0 * lightness) - 1.0).abs()) * saturation;
        let second_component = chroma * (1.0 - (((idx % 2) as f64) - 1.0).abs());
        let (mut r, mut g, mut b) = match idx {
            0 => (chroma, second_component, 0.0),
            1 => (second_component, chroma, 0.0),
            2 => (0.0, chroma, second_component),
            3 => (0.0, second_component, chroma),
            4 => (second_component, 0.0, chroma),
            5 => (chroma, 0.0, second_component),
            _ => unreachable!(),
        };
        let adjustment = lightness - chroma / 2.0;
        r += adjustment;
        g += adjustment;
        b += adjustment;
        Self::rgba(
            (r.min(1.0) * 255.0) as u8,
            (g.min(1.0) * 255.0) as u8,
            (b.min(1.0) * 255.0) as u8,
            (alpha * 255.0) as u8,
        )
    }

    /// Get the r value
    pub fn r(self) -> u8 {
        ((self.data & 0x00FF_0000) >> 16) as u8
    }

    /// Get the g value
    pub fn g(self) -> u8 {
        ((self.data & 0x0000_FF00) >> 8) as u8
    }

    /// Get the b value
    pub fn b(self) -> u8 {
        (self.data & 0x0000_00FF) as u8
    }

    /// Get the alpha value
    pub fn a(self) -> u8 {
        ((self.data & 0xFF00_0000) >> 24) as u8
    }

    /// Attempts to get a color from its name, all the CSS colors are avaible and some other ones also.
    pub fn from_name(name: &str) -> Option<Color> {
        COLORS.get(name).cloned()
    }

    /// Interpolate between two colors
    pub fn interpolate(start_color: Color, end_color: Color, scale: f64) -> Color {
        let r = Color::interp(start_color.r(), end_color.r(), scale);
        let g = Color::interp(start_color.g(), end_color.g(), scale);
        let b = Color::interp(start_color.b(), end_color.b(), scale);
        let a = Color::interp(start_color.a(), end_color.a(), scale);
        Color::rgba(r, g, b, a)
    }

    fn interp(start_color: u8, end_color: u8, scale: f64) -> u8 {
        (end_color as f64 - start_color as f64).mul_add(scale, start_color as f64) as u8
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        if self.a() == 0 {
            return String::from("transparent");
        }

        let data = self.data;

        let mut color = format!("#{:x}", data);
        color.remove(1);
        color.remove(1);
        color
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Color {
        if !s.starts_with('#') {
            if let Some(color) = Color::from_name(s) {
                return color;
            }
        }
        let clean_hex = s.trim_start_matches('#');
        match clean_hex.len() {
            3 | 4 => {
                let num = u32::from_str_radix(&clean_hex, 16).unwrap_or(0);

                let mut blue = (num & 0xF) << 4;
                let mut green = ((num >> 4) & 0xF) << 4;
                let mut red = ((num >> 8) & 0xF) << 4;
                let mut alpha = match clean_hex.len() == 4 {
                    true => ((num >> 12) & 0xF) << 4,
                    false => 0xF,
                };
                red |= red >> 4;
                green |= green >> 4;
                blue |= blue >> 4;
                alpha |= alpha >> 4;

                Color::rgba(red as u8, green as u8, blue as u8, alpha as u8)
            }
            6 | 8 => {
                let mut x = u32::from_str_radix(&clean_hex, 16).unwrap_or(0);

                if clean_hex.len() == 6 {
                    x |= 0xFF00_0000;
                }

                Color { data: x }
            }
            _ => Color { data: 0 },
        }
    }
}

impl From<String> for Color {
    fn from(s: String) -> Color {
        Color::from(s.as_str())
    }
}

/// Compares two colors (the alpha value is ignored)
impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r() == other.r() && self.g() == other.g() && self.b() == other.b()
    }
}

#[cfg(not(feature = "no_std"))]
impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:#010X}", { self.data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partial_eq() {
        assert_eq!(true, Color::rgb(1, 2, 3) == Color::rgba(1, 2, 3, 200));
        assert_eq!(false, Color::rgb(1, 2, 3) == Color::rgba(11, 2, 3, 200));
        assert_eq!(true, Color::rgba(1, 2, 3, 200) == Color::rgba(1, 2, 3, 200));
    }
}
