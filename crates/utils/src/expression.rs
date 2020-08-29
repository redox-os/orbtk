use crate::prelude::*;
use std::f64;
use std::iter::Peekable;
use std::ops::Neg;
use std::{convert::TryFrom, str::Chars};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum ExprOrOp {
    Expression(Expression),
    Operator(Operator),
}

impl ExprOrOp {
    pub fn expression(&self) -> Option<&Expression> {
        match self {
            Self::Expression(e) => Some(e),
            _ => None,
        }
    }
}

// Describes a RON declared function.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Expression {
    Method(String, Vec<Expression>),
    Complex(Vec<ExprOrOp>),
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
                        } else if v.floor() == 0.0 {
                            v = 255.0 * v.fract();
                        }
                    } else if i != 0 && v > 1.0 {
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
            Expression::Other(s) => color_from_name(s),
            _ => None,
        }
    }

    pub fn angle(&self) -> Option<f64> {
        match self {
            Expression::Number(num, unit) => {
                let num: f64 = (*num).into();
                let mut angle = match &unit[..] {
                    "rad" => num,
                    "turn" => f64::consts::PI * 2.0 * num,
                    "" | "deg" => num * f64::consts::PI / 180.0,
                    _ => {
                        return None;
                    }
                };
                if angle.is_sign_negative() {
                    angle = (f64::consts::PI * 2.0) - -angle;
                } else {
                    angle = angle % (f64::consts::PI * 2.0);
                }
                Some(angle)
            }
            _ => None,
        }
    }

    pub fn direction(&self) -> Option<Direction> {
        match self {
            Expression::Other(label) => match &label[..] {
                "to top" => Some(Direction::ToTop),
                "to top right" => Some(Direction::ToTopRight),
                "to right" => Some(Direction::ToRight),
                "to bottom right" => Some(Direction::ToBottomRight),
                "to bottom" => Some(Direction::ToBottom),
                "to bottom left" => Some(Direction::ToBottomLeft),
                "to left" => Some(Direction::ToLeft),
                "to top left" => Some(Direction::ToTopLeft),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn gradient_stop(&self) -> Option<GradientStop> {
        if let Some(color) = self.color() {
            return Some(GradientStop { pos: None, color });
        }
        match self {
            Expression::Complex(v) if v.len() == 2 => {
                let color = match v[0].expression().and_then(|e| e.color()) {
                    Some(color) => color,
                    None => return None,
                };
                let (number, m) = match v[1].expression() {
                    Some(Expression::Number(n, m)) => (*n, m),
                    _ => return None,
                };
                let pos = OnLinePos::try_from((number.into(), &m[..])).ok()?;
                Some(GradientStop {
                    pos: Some(pos),
                    color,
                })
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
                    Some(ExprOrOp::Expression(Expression::Method(name, args))) => {
                        i += 1;
                        (name, args)
                    }
                    _ => {
                        return None;
                    }
                };
                let mut disp_arr = [OnLinePos::default(); 2];
                let mut exprs_idx = i;
                let mut sign = None;
                for arr_idx in 0..2 {
                    match exprs.get(exprs_idx) {
                        Some(ExprOrOp::Operator(Operator::Add)) if sign.is_none() => {
                            sign = Some(true);
                        }
                        Some(ExprOrOp::Operator(Operator::Sub)) if sign.is_none() => {
                            sign = Some(false);
                        }
                        Some(ExprOrOp::Expression(Expression::Number(n, u))) => {
                            let mut pos = OnLinePos::try_from(((*n).into(), &u[..])).ok()?;
                            if let Some(sign) = sign.take() {
                                if !sign {
                                    pos = -pos;
                                }
                            }
                            disp_arr[arr_idx] = pos;
                        }
                        None => break,
                        _ => {
                            return None;
                        }
                    }
                    exprs_idx += 1;
                }
                if exprs.len() == 4 && exprs_idx != 3 {
                    return None;
                }
                *displacement.x_mut() = disp_arr[0];
                *displacement.y_mut() = disp_arr[1];
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
            let mut g = RadialGradient::default();
            if let Expression::Other(ref s) = args[0] {
                match &s[..] {
                    "ellipse" => {
                        g.size = RadialGradientSize::ToClosestSide(false);
                    }
                    "circle" => {
                        g.size = RadialGradientSize::ToClosestSide(true);
                    }
                    _ => {}
                }
            }
            match args[0] {
                Expression::Complex(ref c) if !c.is_empty() => {
                    let mut i = 0;
                    let mut force_circle = false;
                    // Shape definition
                    match c[i] {
                        ExprOrOp::Expression(Expression::Other(ref s)) => {
                            i += 1;
                            match &s[..] {
                                "ellipse" => {
                                    let mut size = [OnLinePos::default(); 2];
                                    let mut with_size = true;
                                    for j in 0..2 {
                                        match c.get(i) {
                                            Some(ExprOrOp::Expression(Expression::Number(
                                                n,
                                                u,
                                            ))) => {
                                                size[j] =
                                                    OnLinePos::try_from(((*n).into(), &u[..]))
                                                        .ok()?;
                                                i += 1;
                                            }
                                            _ if j == 0 => {
                                                with_size = false;
                                            }
                                            _ => {
                                                return None;
                                            }
                                        }
                                    }
                                    if with_size {
                                        g.size = RadialGradientSize::Custom(OnPlanePos::new(
                                            size[0], size[1],
                                        ));
                                    } else {
                                        g.size = RadialGradientSize::ToClosestSide(false);
                                    }
                                }
                                "circle" => {
                                    force_circle = true;
                                    match c.get(i) {
                                        Some(ExprOrOp::Expression(Expression::Number(n, u))) => {
                                            g.size = RadialGradientSize::Radius(
                                                OnLinePos::try_from(((*n).into(), &u[..])).ok()?,
                                            );
                                            i += 1;
                                        }
                                        _ => {
                                            g.size = RadialGradientSize::ToClosestSide(true);
                                        }
                                    }
                                }
                                _ => {
                                    i -= 1;
                                }
                            }
                        }
                        _ => {}
                    }
                    if let Some(ExprOrOp::Expression(Expression::Other(ref o))) = c.get(i) {
                        let mut o = &o[..];
                        if i <= 1 {
                            if o.starts_with("closest-side") {
                                g.size = RadialGradientSize::ToClosestSide(force_circle);
                                o = &o[o.len() - 1 - 2..];
                            } else if o.starts_with("closest-corner") {
                                g.size = RadialGradientSize::ToClosestCorner(force_circle);
                                o = &o[o.len() - 1 - 2..];
                            } else if o.starts_with("farthest-side") {
                                g.size = RadialGradientSize::ToFarthestSide(force_circle);
                                o = &o[o.len() - 1 - 2..];
                            } else if o.starts_with("farthest-corner") {
                                g.size = RadialGradientSize::ToFarthestCorner(force_circle);
                                o = &o[o.len() - 1 - 2..];
                            }
                        }
                        // Position definition
                        if o == "at" {
                            i += 1;
                            let mut sign = None;
                            let mut res_arr = [OnLinePos::default(); 2];
                            for arr_idx in 0..2 {
                                match c.get(i) {
                                    Some(ExprOrOp::Operator(Operator::Add)) if sign.is_none() => {
                                        sign = Some(true);
                                    }
                                    Some(ExprOrOp::Operator(Operator::Sub)) if sign.is_none() => {
                                        sign = Some(false);
                                    }
                                    Some(ExprOrOp::Expression(Expression::Number(n, u))) => {
                                        let mut pos =
                                            OnLinePos::try_from(((*n).into(), &u[..])).ok()?;
                                        if let Some(sign) = sign.take() {
                                            if !sign {
                                                pos = -pos;
                                            }
                                        }
                                        res_arr[arr_idx] = pos;
                                    }
                                    None => break,
                                    _ => {
                                        return None;
                                    }
                                }
                                i += 1;
                            }
                            g.pos = Some(OnPlanePos::new(res_arr[0], res_arr[1]));
                        } else if !o.is_empty() {
                            return None;
                        }
                    }
                }
                _ => {}
            }
            kind = GradientKind::Radial(g);
        } else {
            let mut coords = LinearGradientCoords::Angle {
                radians: 0.0,
                displacement,
            };
            if let Some(direction) = args[0].direction() {
                coords = LinearGradientCoords::Direction {
                    direction,
                    displacement,
                };
                i += 1;
            } else if let Some(radians) = args[0].angle() {
                coords = LinearGradientCoords::Angle {
                    radians,
                    displacement,
                };
                i += 1;
            }
            kind = GradientKind::Linear(coords);
        }
        let mut stops = Vec::new();
        for i in i..args.len() {
            let stop = match args[i].gradient_stop() {
                Some(stop) => stop,
                None => continue,
            };
            stops.push(stop);
        }
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

impl Into<Number> for Expression {
    fn into(self) -> Number {
        match self {
            Expression::Number(num, _) => num,
            _ => Number::default(),
        }
    }
}

pub(crate) fn parse_expression_with_complex(chrs: &mut Peekable<Chars>) -> Option<Expression> {
    let mut v = Vec::new();
    loop {
        if let Some(c) = chrs.peek() {
            let c = *c;
            if c == ',' || c == ')' {
                break;
            } else if c.is_whitespace() {
                // Ignore whitespaces
                chrs.next().unwrap();
                continue;
            } else if c == '+' {
                v.push(ExprOrOp::Operator(Operator::Add));
                chrs.next().unwrap();
                continue;
            } else if c == '-' {
                v.push(ExprOrOp::Operator(Operator::Sub));
                chrs.next().unwrap();
                continue;
            } else if c == '*' {
                v.push(ExprOrOp::Operator(Operator::Mul));
                chrs.next().unwrap();
                continue;
            } else if c == '/' {
                v.push(ExprOrOp::Operator(Operator::Div));
                chrs.next().unwrap();
                continue;
            }
        } else {
            break;
        }
        let mut expr = parse_expression(chrs)?;
        let mut reinterpret_op = None;
        if let Some(ExprOrOp::Operator(op)) = v.last().cloned() {
            if op == Operator::Add || op == Operator::Sub {
                reinterpret_op = Some(op == Operator::Add);
                if v.len() >= 2 {
                    match v[v.len() - 2] {
                        ExprOrOp::Expression(Expression::Number(_, _)) => {
                            // Mathematic expression
                            reinterpret_op = None;
                        }
                        _ => {}
                    };
                }
            }
        }
        if let Some(plus) = reinterpret_op {
            match expr {
                Expression::Number(ref mut n, _) => {
                    v.pop();
                    if !plus {
                        *n = -(*n);
                    }
                }
                _ => {}
            }
        }
        v.push(ExprOrOp::Expression(expr));
    }
    if v.is_empty() {
        None
    } else if v.len() == 1 {
        Some(match v[0] {
            ExprOrOp::Expression(ref e) => e.to_owned(),
            ExprOrOp::Operator(_) => Expression::Complex(v),
        })
    } else {
        Some(Expression::Complex(v))
    }
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
        } else if text.starts_with(|x: char| x.is_ascii_digit() || x == '.' || x == '-') {
            if let Some(mut ofs) = text.rfind(|x: char| x.is_ascii_digit() || x == '.' || x == '-')
            {
                ofs += 1; // Moves from before last position digit to after last digit position
                if text[..ofs]
                    .find(|x| x == '.' || x == 'e' || x == 'E')
                    .is_some()
                {
                    if let Ok(v) = lexical_core::parse(text[..ofs].as_bytes()) {
                        return Some(Expression::Number(Number::Float(v), text[ofs..].to_owned()));
                    }
                } else {
                    if let Ok(v) = lexical_core::parse(text[..ofs].as_bytes()) {
                        return Some(Expression::Number(Number::Real(v), text[ofs..].to_owned()));
                    }
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

    pub fn pixels(&self, size: Size) -> Point {
        Point::from((self.x.pixels(size.width()), self.y.pixels(size.height())))
    }
}

impl Default for OnPlanePos {
    fn default() -> Self {
        OnPlanePos::new(Default::default(), Default::default())
    }
}

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

    pub fn pixels(&self, line_length: f64) -> f64 {
        match self.kind {
            OnLinePosKind::Pixels => self.pos,
            OnLinePosKind::Percentage => line_length * self.pos / 100.0,
        }
    }

    pub fn percent(&self, line_length: f64) -> f64 {
        match self.kind {
            OnLinePosKind::Pixels => self.pos / line_length * 100.0,
            OnLinePosKind::Percentage => self.pos,
        }
    }

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

impl TryFrom<(f64, &str)> for OnLinePos {
    type Error = ();

    fn try_from(value: (f64, &str)) -> Result<Self, Self::Error> {
        let kind = OnLinePosKind::try_from(value.1)?;
        Ok(OnLinePos { pos: value.0, kind })
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum OnLinePosKind {
    Percentage,
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
