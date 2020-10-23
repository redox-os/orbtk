use crate::prelude::*;

/// A `Brush` describes how a shape is filled or stroked.  A `Brush`
/// can be parse from a string (themes are parsing brushes that way).
/// With the given implementation you can choose between three methods
/// to define a solid color:
///
/// A. Use `color codes`
///
/// Define the value with a symbol "#" followed by 6 letters or numbers.
/// These numbers are in hexadecimal numeral system.
/// For example `#f00` will give you red. If you write `#00ff00`, you will
/// get blue and you also included an alpha channel.
///
/// B. Use `a function`
///
/// Currently the unique available functions that interprete a color are
/// destincted with the keywords `rgb`, `hsv`, `hsb`, `hsl`. There are
/// `alpha variants` as well. `hsb` is an alias to `hsv`.
/// Alpha variants are coded with the keywords `rgba`, `abgr`  or `argb`.
/// Here is an example to define a color via the function method:
/// `hsl(197, 71%, 73%)` will provide you a pretty skyblue color.
/// For `rgb` and `rgba` the range of the values are 0-255.
/// Any other keyword will use floating point integers to define the color
/// value. `hsva(0.0-360.0, 0.0-1.0, 0.0-1.0, 0.0-1.0)` is such an example.
/// In addition you can choose to use percent values (`%` sign) for the given
/// parameters.
/// When appending the `%` sign to the range parameters of the `rgb` function
/// call, the values are mapped to 0.0-100.0 (percent) or 0.0-1.0 (min/max).
/// For all other keywords (`hsv`, `hsb`, `hsl`) you are not allowed to append
/// the percent sign to the first parameter. If you append `%` to the following
/// parameters, OrbTk will interpret the values in a range between `0.0-100.0`.
///
/// C. Use the `color name`
///
/// OrbTK maintains a color name map (crates/utils/colors.txt). It enables
/// you, to directly choose from the listed color names. The toolkit will
/// decode the value with the corresponding color code.
/// Example color names are:
///  * white
///  * red
///  * olive,
///  * bombay
///
/// You can also define a gradient with an expression. Its syntax is
/// structured as follows:
/// ```
/// [repeating-]linear-gradient({Gradient-angle}{deg|rad|turn}, ...) [{X Displacement}px {Y Displacement}px]
/// ```
///
/// Within the braces (`{}`) you define the customized parameters.
/// The pipe (`|`) is offering mutual exclusive variants (e.g: degrees(deg),
/// radians(rad) or turns(turn)).
/// The syntax offers optional paramters inside brackets (`[]`).
/// The three points (`...`) refer to multiple stops. They are respected when
/// a gradient is rendered. Use the following syntax:
///
/// ```
/// {Color} [{Stop /// position}{%|px}]
/// ```
///
/// Quite a bit of theory. Lets see some examples:
///
/// ```
/// * linear-gradient(0deg, #4b6cb7, #182848)
/// * repeating-linear-gradient(0.25turn, rgba(255, 255, 0, 0.6), dodgerblue, deepskyblue)
/// * linear-gradient(-90deg, hsv(201, 94%, 80.5%), steelblue)
/// ```
///
/// Oh yes, I do like blue (smile)! Anyway, these examples should
/// introduce the concept and provide some nice implementations using
/// degrees and orientations.
/// You are free to adopt it as appropriate.
#[derive(Clone, PartialEq, Debug)]
pub enum Brush {
    /// Paints an area with a solid color.
    SolidColor(Color),

    /// Paints an area with a gradient.
    Gradient(Gradient),
}

impl Brush {
    pub fn is_transparent(&self) -> bool {
        match self {
            Brush::SolidColor(color) => color.a() == 0,
            _ => false,
        }
    }
}

impl From<Brush> for Color {
    fn from(b: Brush) -> Color {
        match b {
            Brush::SolidColor(color) => color,
            _ => Color::rgb(0, 0, 0),
        }
    }
}

impl From<Brush> for Gradient {
    fn from(b: Brush) -> Gradient {
        match b {
            Brush::Gradient(g) => g,
            _ => Gradient::default(),
        }
    }
}

impl Default for Brush {
    fn default() -> Self {
        Brush::SolidColor(Color::rgba(0, 0, 0, 0))
    }
}

impl From<Color> for Brush {
    fn from(c: Color) -> Brush {
        Brush::SolidColor(c)
    }
}

impl From<Gradient> for Brush {
    fn from(g: Gradient) -> Brush {
        Brush::Gradient(g)
    }
}

impl From<&str> for Brush {
    fn from(s: &str) -> Brush {
        Expression::from(s).brush().unwrap_or_default()
    }
}

impl From<String> for Brush {
    fn from(s: String) -> Brush {
        Self::from(&s[..])
    }
}

impl From<Value> for Brush {
    fn from(v: Value) -> Self {
        let value = v.get::<String>();
        Brush::from(value)
    }
}

#[cfg(test)]
mod tests {
    //  use crate::prelude::*;
    // todo: tbd after brush struct is finished
}
