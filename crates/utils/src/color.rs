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
    pub fn hsva(mut h: f64, mut s: f64, mut v: f64, a: f64) -> Self {
        h = h % 360.0;
        s = s.max(0.0).min(1.0);
        v = v.max(0.0).min(1.0);
        dbg!((h, s, v));
        let hh = h / 60.0;
        let i = hh.floor() as i32;
        let ff = hh.fract();
        let p = v * (1.0 - s);
        let q = v * (1.0 - (s * ff));
        let t = v * (1.0 - (s * (1.0 - ff)));
        let (r, g, b) = match i {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            5 => (v, p, q),
            _ => unreachable!(),
        };
        Self::rgba(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            (a * 255.0) as u8,
        )
    }

    /// Create a new color from HSL(0.0-360.0, 0.0-1.0, 0.0-1.0) and alpha values(0.0-1.0)
    pub fn hsla(mut h: f64, mut s: f64, mut l: f64, a: f64) -> Self {
        h = h % 360.0;
        s = s.max(0.0).min(1.0);
        l = l.max(0.0).min(1.0);
        let hh = h / 60.0;
        let i = hh.floor() as i32;
        let chroma = (1.0 - ((2.0 * l) - 1.0).abs()) * s;
        let second_component = chroma * (1.0 - (((i % 2) as f64) - 1.0).abs());
        let (mut r, mut g, mut b) = match i {
            0 => (chroma, second_component, 0.0),
            1 => (second_component, chroma, 0.0),
            2 => (0.0, chroma, second_component),
            3 => (0.0, second_component, chroma),
            4 => (second_component, 0.0, chroma),
            5 => (chroma, 0.0, second_component),
            _ => unreachable!(),
        };
        let adjustment = l - chroma / 2.0;
        r += adjustment;
        g += adjustment;
        b += adjustment;
        Self::rgba(
            (r.min(1.0) * 255.0) as u8,
            (g.min(1.0) * 255.0) as u8,
            (b.min(1.0) * 255.0) as u8,
            (a * 255.0) as u8,
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
        let clean_hex = s.trim_start_matches('#');
        match clean_hex.len() {
            3 | 4 => {
                let d = match u32::from_str_radix(&clean_hex, 16) {
                    Ok(x) => x,
                    Err(_) => 0,
                };

                let mut b = (d & 0xF) << 4;
                let mut g = ((d >> 4) & 0xF) << 4;
                let mut r = ((d >> 8) & 0xF) << 4;
                let mut a = match clean_hex.len() == 4 {
                    true => ((d >> 12) & 0xF) << 4,
                    false => 0xF,
                };
                r |= r >> 4;
                g |= g >> 4;
                b |= b >> 4;
                a |= a >> 4;

                Color::rgba(r as u8, g as u8, b as u8, a as u8)
            }
            6 | 8 => {
                let mut x = match u32::from_str_radix(&clean_hex, 16) {
                    Ok(x) => x,
                    Err(_) => 0,
                };

                if clean_hex.len() == 6 {
                    x |= 0xFF_000_000;
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
