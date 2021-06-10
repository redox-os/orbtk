use crate::prelude::*;

/// A `Brush` defines the fill pattern of shapes.
/// The syntax allows to express fill patterns in several ways:
///
/// * solid colors
/// * colors with alpha channel
/// * gradients of colors
/// * gradients with directions
/// * gradients with angles
///
/// The string declaration of a `Brush` is composed combining the following
/// syntax elements:
///
/// 1. The `color name`
/// 2. The `gradient` string
///    * the gradient type (linear, repeating-linear)
///    * gradient attributes (direction-identifier, angles, color names )
///
/// ## Examples
/// Here are some implementations with declarations of colors, degrees, orientations and directions.
///
/// ```text
/// .foreground("white")
/// .background("black")
/// .background("linear-gradient(0deg, #4b6cb7, #182848)")
/// .background("repeating-linear-gradient(0.25turn, rgba(255, 255, 0, 0.6), dodgerblue, deepskyblue)")
/// .background("linear-gradient(-90deg, hsv(201, 94%, 80.5%), steelblue)")
/// .background("linear-gradient(to top right, white, skyblue 60%, lightskyblue 80%, yellow 83%, yellow)")
/// ```
/// Read on to see how the syntax is composed.
///
/// ## Definition of a color name
/// With the given implementation you can choose between three methods
/// to define a color.
///
/// A. `color codes`
///
/// You can define the value of a color with a symbol "#" followed
/// by letters or numbers. These numbers are in hexadecimal numeral system.
/// The short variant will use 3 numbers , the long variant will use 6
/// numbers.
/// For example `#f00` will give you red. If you write `#0000ff`, you will
/// get blue.
/// To include an alpha channel, the short variant takes 4 numbers.
/// If you need a yellow with 50.2% opaque, you use `#ff08`.
/// In the long form you need 8 numbers. `#0000ff80` represents 50.2% opaque
/// (non-premultiplied) blue.
///
/// B. `color function`
///
/// Currently the unique available functions that interpret a color are
/// distincted with the keywords `rgb`, `hsv`, `hsb`, `hsl`. There are
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
/// C. `color name`
///
/// **WIP: The given implementation is using (utils/colors.txt). This has to be adopted!!!**
///
/// OrbTK maintains color names as constants [`utils::const_colors`]. It enables
/// you, to directly choose their string value inside the code.
/// Example color names are:
///
///  * COLOR_WHITE
///  * COLOR_RED
///  * COLOR_OLIVE
///  * COLOR_LINK_WATER
///  * COLOR_SLATE_GRAY
///
/// ## Definition of a gradient
/// The syntax of a gradient definition is structured as follows:
///
/// * Optional parameters are inside brackets (`[]`).
/// * Within braces (`{}`) you define the appropriate parameter value.
/// * The pipe (`|`) is offering mutual exclusive variants
///   e.g: degrees(deg), radians(rad) or turns(turn).
/// * Three points (`...`) refer to multiple stops.
///   They are respected when a gradient is rendered.
///
/// To understand gradient directions, imagine a line or vector that
/// starts at a given point inside the entity and points to an
/// imaginary target point within the same entity. Gradients will be
/// rendered along the choosen direction to reach its target
/// poing. Supported directions are:
///
///  * "to bottom"
///  * "to bottom left"
///  * "to bottom right"
///  * "to left"
///  * "to right"
///  * "to top
///  * "to top left"
///  * "to top right"
///
/// Displacement points tell the gradient algorithm to add
/// (`positive`) or or substract (`negative`) the given pixel numbers
/// from the original starting point.
///
/// Lets look at some examples. The first one shows the
/// structure of an angled gradient
///
/// ```text
/// [repeating-]linear-gradient({Gradient-angle}{deg|rad|turn}, ...) [{X Displacement}px {Y Displacement}px], {Color} [{Stop  position}{%|px}]
/// ```
///
/// The next example shows the structure of a gradient that will be
/// rendered in a given direction
///
/// ```text
/// [repeating-]linear-gradient({direction-identifier}, {initial color-name}, {terminating color-name}
/// ```
///
//#[cfg(feature = "nightly")]
//#[doc(include = "../colors.md")]

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
