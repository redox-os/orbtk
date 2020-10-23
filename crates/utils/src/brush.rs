use crate::prelude::*;

/// A `Brush` describes how a shape is filled or stroked.
/// A `Brush` can be parse from a string(as that is how the brushes are parsed in the themes)
/// For example by an expression you have three methods to define a solid color:
///
/// A. Use color codes, for example `#f00` will give you red or if you write #00ff00 you will got blue
/// also you can use alpha.
///
/// B. Use a function, currently the unique available functions to interpret a color are `rgb`, `hsv`, `hsb`
/// (which is the same as `hsv`), `hsl` and its respective alpha variants(rgba by example). So you can define
/// a color as `hsl(197, 71%, 73%)` and you will get a pretty skyblue color. For `rgb` and `rgba` the
/// range of the values are 0-255 but from all others the values are as follows: hsva(0.0-360.0, 0.0-1.0, 0.0-1.0, 0.0-1.0)
/// but the % sign can change that, it can be added to individual parameters and change its range depending on the function,
/// for `rgb` and `rgba` you can write an % after a parameter and it will be mapped from 0.0-100.0, or if the number that you write
/// is between 0.0 and 1.0 you value will be mapped from that range. But if you function is not rgb or rgba, you can not use a
/// percent sign in the first parameters but in the following parameters the sign will make OrbTk interpret it in the 0.0-100.0 range.
///
/// C. Use the color name! so you want to write a colour and do you code work lazily?, no problem! you can directly write the name.
/// For example white, red, olive, ivory, linen or any of the CSS color names!, also the next extra colors are supported:
/// * alabaster
/// * alto
/// * bluebayoux
/// * bombay
/// * brightgrey
/// * dolly
/// * energyellow
/// * fiord
/// * goldendream
/// * goldenfizz
/// * goldtips
/// * gorduroy
/// * governorbay
/// * linkwater
/// * lynch
/// * manatee
/// * manz
/// * mineshaft
/// * periwinklegrey
/// * portage
/// * sandwisp
/// * silverchalice
/// * sunflower
/// You also can define a gradient with an expression, the syntax is as follows:
/// ```
/// [repeating-]linear-gradient({Gradient angle}{deg|rad|turn}, ...) [{X Displacement}px {Y Displacement}px]
/// ```
/// Where {} speaks of a customizable parameter except if it has a | in middle then is only
/// a multiple choice question(so you have to choice between degrees(deg), radians(rad) or
/// turns(turn)) and [] means and optional part(but with a meaning!), now the ... are only
/// referring to the multiples stops that you can define in your gradient, the syntax for a
/// stop is as follows:
/// ```
/// {Color} [{Stop position}{%|px}]
/// ```
/// But enough theory, here are some examples:
/// ```
/// linear-gradient(0deg, #4b6cb7, #182848)
/// repeating-linear-gradient(0.25turn, rgba(255, 255, 0, 0.6), dodgerblue, deepskyblue)
/// linear-gradient(-90deg, hsv(201, 94%, 80.5%), steelblue)
/// ```
/// Yes sir, I like the blue but that examples works and give you some nice degrees and orientation
/// about how to use it.
/// I expected this quick explain will help you, thanks for reading!
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
