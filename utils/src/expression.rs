use crate::prelude::*;
use std::f64;
use std::iter::Peekable;
use std::ops::Neg;
use std::{convert::TryFrom, str::Chars};

// Describes a String declared expression either be a method, a color, a number or anything.
/// This object represents a `expression` used to define something(currently is only use to define
/// brushes in themes but that can change in the future.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Expression {
    Method(String, Vec<Expression>),
    Complex(Vec<Expression>),
    Number(Number, String),
    Color(Color),
    Other(String),
}

impl Expression {
    /// Try to convert `self` into a `Number`
    pub fn number(&self) -> Option<Number> {
        match self {
            Expression::Number(number, d) if d.is_empty() => Some(*number),
            _ => None,
        }
    }

    pub fn color(&self) -> Option<Color> {
        match self {
            Expression::Color(color) => Some(*color),
            Expression::Method(name, args) => {
                let mut values = [0.0f64; 4];
                for (i, arg) in args.iter().enumerate() {
                    if i > 3 {
                        return None;
                    }
                    let (mut v, p): (f64, bool) = match arg {
                        Expression::Number(v, u) if u.is_empty() => ((*v).into(), false),
                        Expression::Number(v, u) if u == "%" => ((*v).into(), true),
                        _ => {
                            return None;
                        }
                    };
                    if name == "rgb" || name == "rgba" {
                        if p {
                            v = v * 100.0 / 255.0;
                        } else if v <= 1.0 {
                            v = 255.0 * v.fract();
                        }
                    } else if i != 0 && p {
                        v /= 100.0;
                    }
                    values[i] = v;
                }
                if args.len() == 3 {
                    Some(match &name[..] {
                        "rgb" => Color::rgb(values[0] as u8, values[1] as u8, values[2] as u8),
                        "hsv" | "hsb" => Color::hsv(values[0], values[1], values[2]),
                        "hsl" => Color::hsl(values[0], values[1], values[2]),
                        _ => return None,
                    })
                } else {
                    Some(match &name[..] {
                        "rgba" => Color::rgba(
                            values[0] as u8,
                            values[1] as u8,
                            values[2] as u8,
                            values[3] as u8,
                        ),
                        "hsva" | "hsba" => Color::hsva(values[0], values[1], values[2], values[3]),
                        "hsla" => Color::hsla(values[0], values[1], values[2], values[3]),
                        _ => return None,
                    })
                }
            }
            Expression::Other(s) => Color::from_name(s),
            _ => None,
        }
    }

    fn gradient_stop(&self) -> Option<GradientStop> {
        if let Some(color) = self.color() {
            return Some(GradientStop { pos: None, color });
        }
        match self {
            Expression::Complex(v) if v.len() == 2 => {
                let color = match v[0].color() {
                    Some(color) => color,
                    None => return None,
                };
                let pos = match v[1] {
                    Expression::Number(n, ref m) => OnLinePos::try_from((n, &m[..])).ok()?,
                    _ => return None,
                };
                Some(GradientStop {
                    pos: Some(pos),
                    color,
                })
            }
            _ => None,
        }
    }

    pub fn relative_dir(&self) -> Option<RelativeDir> {
        match self {
            Expression::Other(label) => match &label[..] {
                "to top" => Some(RelativeDir::Top),
                "to top right" => Some(RelativeDir::TopRight),
                "to right" => Some(RelativeDir::Right),
                "to bottom right" => Some(RelativeDir::BottomRight),
                "to bottom" => Some(RelativeDir::Bottom),
                "to bottom left" => Some(RelativeDir::BottomLeft),
                "to left" => Some(RelativeDir::Left),
                "to top left" => Some(RelativeDir::TopLeft),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn angle(&self) -> Option<Angle> {
        match self {
            Expression::Number(num, unit) => {
                let num: f64 = (*num).into();
                let angle = match &unit[..] {
                    "rad" => Angle::from_radians(num),
                    "turn" => Angle::from_turn(num),
                    "deg" | "" => Angle::from_degrees(num),
                    _ => {
                        return None;
                    }
                };
                Some(angle)
            }
            _ => None,
        }
    }

    pub fn css_gradient(&self) -> Option<Gradient> {
        let mut displacement = OnPlanePos::new(
            OnLinePos::new(0.0, OnLinePosKind::Pixels),
            OnLinePos::new(0.0, OnLinePosKind::Pixels),
        );
        let (name, args) = match self {
            Expression::Method(name, args) => (name, args),
            Expression::Complex(exprs) if exprs.len() <= 3 + 1 => {
                let mut i = 0;
                let (name, args) = match exprs.get(i) {
                    Some(Expression::Method(name, args)) => {
                        i += 1;
                        (name, args)
                    }
                    _ => {
                        return None;
                    }
                };
                *displacement.x_mut() = match exprs.get(i) {
                    Some(Expression::Number(n, u)) => {
                        i += 1;
                        OnLinePos::try_from((*n, &u[..])).ok()?
                    }
                    _ => {
                        return None;
                    }
                };
                *displacement.y_mut() = match exprs.get(i) {
                    Some(Expression::Number(n, u)) => OnLinePos::try_from((*n, &u[..])).ok()?,
                    _ => {
                        return None;
                    }
                };
                (name, args)
            }
            _ => return None,
        };
        if args.is_empty() {
            return None;
        }
        let (radial, repeat) = match &name[..] {
            "repeating-linear-gradient" => (false, true),
            "linear-gradient" => (false, false),
            "radial-gradient" => (true, false),
            "repeating-radial-gradient" => (true, true),
            _ => {
                return None;
            }
        };
        let mut i = 0;
        let kind;
        if radial {
            // TODO: Implement radial gradients
            return None;
        } else {
            let mut coords = LinearGradientCoords::Angle {
                displacement,
                angle: Angle::zero(),
            };
            if let Some(direction) = args[0].relative_dir() {
                coords = LinearGradientCoords::Direction {
                    direction,
                    displacement,
                };
            } else if let Some(angle) = args[0].angle() {
                coords = LinearGradientCoords::Angle {
                    angle,
                    displacement,
                };
                i += 1;
            }
            kind = GradientKind::Linear(coords);
        }
        let stops: Vec<GradientStop> = args
            .iter()
            .skip(i)
            .filter_map(|stop| stop.gradient_stop())
            .collect();
        if stops.is_empty() {
            return None;
        }
        Some(Gradient {
            kind,
            stops,
            repeat,
        })
    }

    pub fn brush(&self) -> Option<Brush> {
        if let Some(color) = self.color() {
            return Some(Brush::from(color));
        }
        if let Some(g) = self.css_gradient() {
            return Some(Brush::from(g));
        }
        None
    }
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Complex(Vec::new())
    }
}

impl From<Expression> for Number {
    fn from(e: Expression) -> Self {
        match e {
            Expression::Number(num, _) => num,
            _ => Number::default(),
        }
    }
}

pub(crate) fn parse_expression_with_complex(chrs: &mut Peekable<Chars>) -> Option<Expression> {
    let mut v = Vec::new();
    while let Some(c) = chrs.peek() {
        let c = *c;
        if c == ',' || c == ')' {
            break;
        } else if c.is_whitespace() {
            // Ignore whitespaces
            chrs.next().unwrap();
            continue;
        }
        let expr = parse_expression(chrs)?;
        v.push(expr);
    }
    if v.is_empty() {
        None
    } else if v.len() == 1 {
        Some(v[0].to_owned())
    } else {
        Some(Expression::Complex(v))
    }
}

fn is_number_component(c: char) -> bool {
    c.is_ascii_digit() || c == '.' || c == '-'
}

fn parse_expression(chrs: &mut Peekable<Chars>) -> Option<Expression> {
    let mut text = String::new();
    let method;
    loop {
        match chrs.peek() {
            Some('(') => {
                chrs.next().unwrap();
                method = true;
                break;
            }
            Some(c) if *c == ',' || *c == ')' || (c.is_whitespace() && text != "to") => {
                method = false;
                break;
            }
            Some(c) => {
                text.push(*c);
                chrs.next().unwrap();
            }
            None => {
                method = false;
                break;
            }
        }
    }
    debug_assert!(!text.is_empty());
    if method {
        let mut args = Vec::new();
        loop {
            match chrs.peek() {
                Some(c) if c.is_whitespace() || *c == ',' => {
                    chrs.next().unwrap();
                }
                None | Some(')') => {
                    let _ = chrs.next();
                    break;
                }
                _ => {
                    args.push(parse_expression_with_complex(chrs)?);
                }
            }
        }
        Some(Expression::Method(text, args))
    } else {
        if text.starts_with('#') {
            return Some(Expression::Color(Color::from(text)));
        } else if text.starts_with(is_number_component) {
            if let Some(mut ofs) = text.rfind(is_number_component) {
                ofs += 1; // Moves from before last position digit to after last digit position
                if text[..ofs]
                    .find(|x| x == '.' || x == 'e' || x == 'E')
                    .is_some()
                {
                    if let Ok(v) = lexical_core::parse(text[..ofs].as_bytes()) {
                        return Some(Expression::Number(Number::Float(v), text[ofs..].to_owned()));
                    }
                } else if let Ok(v) = lexical_core::parse(text[..ofs].as_bytes()) {
                    return Some(Expression::Number(Number::Real(v), text[ofs..].to_owned()));
                }
            }
        }
        Some(Expression::Other(text))
    }
}

impl From<&str> for Expression {
    fn from(s: &str) -> Expression {
        parse_expression_with_complex(&mut s.chars().peekable()).unwrap_or_default()
    }
}

impl From<String> for Expression {
    fn from(s: String) -> Expression {
        Expression::from(&s[..])
    }
}

/// Describes a position on a plane
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct OnPlanePos {
    x: OnLinePos,
    y: OnLinePos,
}

impl OnPlanePos {
    pub fn new(x: OnLinePos, y: OnLinePos) -> OnPlanePos {
        OnPlanePos { x, y }
    }

    pub fn x(&self) -> OnLinePos {
        self.x
    }

    pub fn y(&self) -> OnLinePos {
        self.y
    }

    pub fn x_mut(&mut self) -> &mut OnLinePos {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut OnLinePos {
        &mut self.y
    }

    /// Returns the position in pixels
    pub fn pixels(&self, size: Size) -> Point {
        Point::from((self.x.pixels(size.width()), self.y.pixels(size.height())))
    }

    /// Returns the position in percent
    pub fn percent(&self, size: Size) -> Point {
        Point::from((self.x.percent(size.width()), self.y.percent(size.height())))
    }

    /// Returns the position in a range from 0.0 to 1.0
    pub fn unit_percent(&self, size: Size) -> Point {
        Point::from((
            self.x.unit_percent(size.width()),
            self.y.unit_percent(size.height()),
        ))
    }
}

impl Default for OnPlanePos {
    fn default() -> Self {
        OnPlanePos::new(Default::default(), Default::default())
    }
}

/// Describes a position on a line
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct OnLinePos {
    pos: f64,
    kind: OnLinePosKind,
}

impl OnLinePos {
    pub fn new(pos: f64, kind: OnLinePosKind) -> OnLinePos {
        OnLinePos { pos, kind }
    }

    pub fn from_unit_percent(pos: f64) -> OnLinePos {
        Self::new(pos * 100.0, OnLinePosKind::Percentage)
    }

    pub fn pos(&self) -> f64 {
        self.pos
    }

    /// Returns the position in pixels
    pub fn pixels(&self, line_length: f64) -> f64 {
        match self.kind {
            OnLinePosKind::Pixels => self.pos,
            OnLinePosKind::Percentage => line_length * self.pos / 100.0,
        }
    }

    /// Returns the position in percent
    pub fn percent(&self, line_length: f64) -> f64 {
        match self.kind {
            OnLinePosKind::Pixels => self.pos / line_length * 100.0,
            OnLinePosKind::Percentage => self.pos,
        }
    }

    /// Returns the position in a range from 0.0 to 1.0
    pub fn unit_percent(&self, line_length: f64) -> f64 {
        self.percent(line_length) / 100.0
    }
}

impl Default for OnLinePos {
    fn default() -> Self {
        Self {
            pos: 0.0,
            kind: OnLinePosKind::default(),
        }
    }
}

impl<N> TryFrom<(N, &str)> for OnLinePos
where
    N: Into<f64>,
{
    type Error = ();

    fn try_from(value: (N, &str)) -> Result<Self, Self::Error> {
        let kind = OnLinePosKind::try_from(value.1)?;
        Ok(OnLinePos {
            pos: (value.0).into(),
            kind,
        })
    }
}

/// This only is used to communicate the kind of `OnLinePos` we are using
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum OnLinePosKind {
    /// A number from 0.0 to 100.0
    Percentage,
    /// This tells to you that `OnLinePos` is storing the position in pixels directly
    Pixels,
}

impl TryFrom<&str> for OnLinePosKind {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "px" => Ok(OnLinePosKind::Pixels),
            "%" => Ok(OnLinePosKind::Percentage),
            _ => Err(()),
        }
    }
}

impl Default for OnLinePosKind {
    fn default() -> Self {
        Self::Pixels
    }
}

impl Neg for OnLinePos {
    type Output = OnLinePos;

    fn neg(mut self) -> Self::Output {
        self.pos = -self.pos;
        self
    }
}
