//! This module contains all css theming releated resources.

use std::{fs::File, io::BufReader, io::Read, mem, path::Path, sync::Arc};

use cssparser::{
    self, BasicParseError, CompactCowStr, DeclarationListParser, ParseError, Parser, ParserInput,
    Token,
};

use orbclient::Color;

pub use self::cell::CloneCell;
use self::selector::Specificity;
pub use self::selector::{Selector, SelectorProperty, SelectorRelation};
pub use self::style::Style;

mod cell;

mod selector;
mod style;

use crate::styling::theme::{DEFAULT_THEME_CSS, LIGHT_THEME_EXTENSION_CSS};

lazy_static! {
    static ref DEFAULT_THEME: Arc<Theme> = {
        Arc::new(Theme {
            parent: None,
            rules: parse(DEFAULT_THEME_CSS),
        })
    };
}

lazy_static! {
    pub static ref LIGHT_THEME_CSS: String =
        format!("{}{}", LIGHT_THEME_EXTENSION_CSS, DEFAULT_THEME_CSS);
}

/// Used to build a theme, specifying additional details.
pub struct ThemeBuilder {
    theme_css: Option<String>,
    theme_path: Option<String>,
    theme_extensitons: Vec<String>,
    theme_exention_paths: Vec<String>,
}

impl Default for ThemeBuilder {
    fn default() -> Self {
        ThemeBuilder {
            theme_css: None,
            theme_path: None,
            theme_extensitons: vec![],
            theme_exention_paths: vec![],
        }
    }
}

impl ThemeBuilder {
    /// Inserts a theme css extension.
    pub fn extenstion_css(mut self, extension: impl Into<String>) -> Self {
        self.theme_extensitons.push(extension.into());
        self
    }

    /// Inserts a theme extension by path.
    pub fn extenstion_path(mut self, extension_path: impl Into<String>) -> Self {
        self.theme_exention_paths.push(extension_path.into());
        self
    }

    /// Builds the theme.
    pub fn build(self) -> Theme {
        let mut theme = String::new();

        for css_extension in self.theme_extensitons.iter().rev() {
            theme.push_str(&css_extension);
        }

        for extension_path in self.theme_exention_paths.iter().rev() {
            let file = File::open(extension_path).unwrap();

            let mut reader = BufReader::new(file);
            let mut css = String::new();
            let _ = reader.read_to_string(&mut css).unwrap();

            theme.push_str(&css);
        }

        if let Some(css) = self.theme_css {
            theme.push_str(&css);
        };

        if let Some(path) = self.theme_path {
            let file = File::open(path).unwrap();

            let mut reader = BufReader::new(file);
            let mut css = String::new();
            let _ = reader.read_to_string(&mut css).unwrap();

            theme.push_str(&css);
        };

        Theme::parse(&theme)
    }
}

/// `Theme` is the represenation of a css styling.
#[derive(Debug, Clone)]
pub struct Theme {
    parent: Option<Arc<Theme>>,
    rules: Vec<Rule>,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::parse(DEFAULT_THEME_CSS)
    }
}

impl Theme {
    /// Returns the default light theme.
    pub fn default_light() -> Theme {
        Theme::parse(&format!(
            "{}{}",
            LIGHT_THEME_EXTENSION_CSS, DEFAULT_THEME_CSS
        ))
    }

    /// Creates a new `ThemeBuilder` object with default theme as base.
    pub fn create() -> ThemeBuilder {
        ThemeBuilder {
            theme_css: Some(DEFAULT_THEME_CSS.to_string()),
            ..Default::default()
        }
    }

    /// Creates a new `ThemeBuilder` object with light theme as base.
    pub fn create_light_theme() -> ThemeBuilder {
        ThemeBuilder {
            theme_css: Some(format!(
                "{}{}",
                LIGHT_THEME_EXTENSION_CSS, DEFAULT_THEME_CSS
            )),
            ..Default::default()
        }
    }

    /// Creates a new `ThemeBuilder` with the given css as base.
    pub fn create_from_css(css: impl Into<String>) -> ThemeBuilder {
        ThemeBuilder {
            theme_css: Some(css.into()),
            ..Default::default()
        }
    }

    /// Creates a new `ThemeBuilder` with the given css path as base.
    pub fn create_from_path(path: impl Into<String>) -> ThemeBuilder {
        ThemeBuilder {
            theme_path: Some(path.into()),
            ..Default::default()
        }
    }

    fn parse(s: &str) -> Self {
        Theme {
            parent: Some(DEFAULT_THEME.clone()),
            rules: parse(s),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Theme, String> {
        let file = r#try!(File::open(path).map_err(|err| format!("failed to open css: {}", err)));
        let mut reader = BufReader::new(file);
        let mut css = String::new();
        let res = reader
            .read_to_string(&mut css)
            .map_err(|err| format!("failed to read css: {}", err));
        match res {
            Ok(_) => Ok(Theme::parse(&css)),
            Err(err) => Err(err),
        }
    }

    fn all_rules(&self) -> Vec<Rule> {
        if let Some(ref parent) = self.parent {
            self.rules
                .iter()
                .chain(parent.rules.iter())
                .cloned()
                .collect()
        } else {
            self.rules.clone()
        }
    }

    pub fn get(&self, property: &str, query: &Selector) -> Option<Value> {
        let mut matches: Vec<(bool, Specificity, Value)> = Vec::new();

        for rule in self.all_rules().iter().rev() {
            let matching_selectors = rule
                .selectors
                .iter()
                .filter(|x| x.matches(query))
                .collect::<Vec<_>>();

            if !matching_selectors.is_empty() {
                if let Some(decl) = rule
                    .declarations
                    .iter()
                    .find(|decl| decl.property == property)
                {
                    let highest_specifity = matching_selectors
                        .iter()
                        .map(|sel| sel.specificity())
                        .max()
                        .unwrap();
                    matches.push((decl.important, highest_specifity, decl.value.clone()));
                }
            }
        }

        matches.sort_by_key(|x| (x.0, x.1));
        matches.last().map(|x| x.2.clone())
    }

    pub fn color(&self, property: &str, query: &Selector) -> Color {
        let default = Color { data: 0 };
        self.get(property, query)
            .map(|v| v.color().unwrap_or(default))
            .unwrap_or(default)
    }

    pub fn uint(&self, property: &str, query: &Selector) -> u32 {
        self.get(property, query)
            .map(|v| v.uint().unwrap_or(0))
            .unwrap_or(0)
    }

    pub fn float(&self, property: &str, query: &Selector) -> f32 {
        self.get(property, query)
            .map(|v| v.float().unwrap_or(1.0))
            .unwrap_or(1.0)
    }

    pub fn string(&self, property: &str, query: &Selector) -> String {
        self.get(property, query)
            .map(|v| v.string().unwrap_or(String::default()))
            .unwrap_or(String::default())
    }
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

impl<T: Into<String>> From<T> for Selector {
    fn from(t: T) -> Self {
        Selector::new().with(t.into())
    }
}

#[derive(Clone, Debug)]
pub struct Declaration {
    pub property: String,
    pub value: Value,
    pub important: bool,
}

#[derive(Clone, Debug)]
pub enum Value {
    UInt(u32),
    Float(f32),
    Color(Color),
    Str(String),
}

impl Value {
    pub fn uint(&self) -> Option<u32> {
        match *self {
            Value::UInt(x) => Some(x),
            _ => None,
        }
    }

    pub fn float(&self) -> Option<f32> {
        match *self {
            Value::Float(x) => Some(x),
            _ => None,
        }
    }

    pub fn color(&self) -> Option<Color> {
        match *self {
            Value::Color(x) => Some(x),
            _ => None,
        }
    }

    pub fn string(&self) -> Option<String> {
        match self {
            Value::Str(x) => Some(x.clone()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum CustomParseError {
    InvalidColorName(String),
    InvalidColorHex(String),
    InvalidStringName(String),
}

impl<'t> From<CustomParseError> for ParseError<'t, CustomParseError> {
    fn from(e: CustomParseError) -> Self {
        ParseError::Custom(e)
    }
}

struct RuleParser;

impl RuleParser {
    fn new() -> Self {
        RuleParser {}
    }
}

impl<'i> cssparser::QualifiedRuleParser<'i> for RuleParser {
    type Prelude = Vec<Selector>;
    type QualifiedRule = Rule;
    type Error = CustomParseError;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let res = parse_selectors(input)?;
        Ok(res)
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let decl_parser = DeclarationParser {};

        let decls = DeclarationListParser::new(input, decl_parser).collect::<Vec<_>>();

        for decl in &decls {
            match *decl {
                Ok(_) => {}
                Err(ref e) => {
                    match e.error {
                        ParseError::Basic(ref e) => eprintln!("{:?}", e),
                        ParseError::Custom(ref e) => eprintln!("{:?}", e),
                    }
                    println!("Error occured in `{}`", input.slice(e.span.clone()));
                }
            }
        }

        let decls = decls.into_iter().filter_map(|decl| decl.ok()).collect();

        Ok(Rule {
            selectors: selectors,
            declarations: decls,
        })
    }
}

impl<'i> cssparser::AtRuleParser<'i> for RuleParser {
    type Prelude = ();
    type AtRule = Rule;
    type Error = CustomParseError;
}

fn parse_selectors<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Vec<Selector>, ParseError<'i, CustomParseError>> {
    let mut selectors = Vec::new();

    let mut selector = Selector::default();

    let mut first_token_in_selector = true;
    while let Ok(t) = input.next() {
        match t {
            // Element
            Token::Ident(ref element_name) => {
                if first_token_in_selector {
                    selector.element = Some(element_name.to_string())
                } else {
                    let mut old_selector = Selector::new().with(element_name.to_string());
                    mem::swap(&mut old_selector, &mut selector);
                    selector.relation = Some(Box::new(SelectorRelation::Ancestor(old_selector)));
                }
            }

            Token::Delim('>') => {
                let mut old_selector = Selector::new().with(input.expect_ident()?.to_string());
                mem::swap(&mut old_selector, &mut selector);
                selector.relation = Some(Box::new(SelectorRelation::Parent(old_selector)));
            }

            // Id
            Token::IDHash(ref id_name) => {
                selector.id = Some(id_name.to_string());
            }

            // Any element
            Token::Delim('*') => {}

            // Class
            Token::Delim('.') => {
                selector.classes.insert(input.expect_ident()?.into_owned());
            }

            // Pseudo-class
            Token::Colon => {
                selector
                    .pseudo_classes
                    .insert(input.expect_ident()?.into_owned());
            }

            // This selector is done, on to the next one
            Token::Comma => {
                selectors.push(selector);
                selector = Selector::default();
                first_token_in_selector = true;
                continue; // need to continue to avoid `first_token_in_selector` being set to false
            }

            t => {
                let basic_error = BasicParseError::UnexpectedToken(t);
                return Err(basic_error.into());
            }
        }

        first_token_in_selector = false;
    }

    selectors.push(selector);

    if selectors.iter().any(|sel| sel.relation.is_some()) {
        eprintln!("WARNING: Complex selector relations not implemented");
    }

    Ok(selectors)
}

struct DeclarationParser;

impl<'i> cssparser::DeclarationParser<'i> for DeclarationParser {
    type Declaration = Declaration;
    type Error = CustomParseError;

    fn parse_value<'t>(
        &mut self,
        name: CompactCowStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let value = match &*name {
            "color" | "border-color" | "icon-color" => Value::Color(parse_basic_color(input)?),

            "background" | "foreground" => Value::Color(parse_basic_color(input)?),

            "font-family" | "icon-font-family" => Value::Str(parse_string(input)?),

            "border-radius" | "border-width" | "font-size" | "icon-size" | "icon-margin" => {
                match input.next()? {
                    Token::Number {
                        int_value: Some(x),
                        has_sign,
                        ..
                    } if !has_sign && x >= 0 => Value::UInt(x as u32),
                    t => return Err(BasicParseError::UnexpectedToken(t).into()),
                }
            }

            "opacity" => match input.next()? {
                Token::Number { value: x, .. } => Value::Float(x as f32),
                t => return Err(BasicParseError::UnexpectedToken(t).into()),
            },

            _ => return Err(BasicParseError::UnexpectedToken(input.next()?).into()),
        };

        Ok(Declaration {
            property: name.into_owned(),
            value: value,
            important: input.r#try(cssparser::parse_important).is_ok(),
        })
    }
}

impl<'i> cssparser::AtRuleParser<'i> for DeclarationParser {
    type Prelude = ();
    type AtRule = Declaration;
    type Error = CustomParseError;
}

fn css_color(name: &str) -> Option<Color> {
    Some(hex(match name {
        "transparent" => return Some(Color { data: 0 }),

        "black" => 0x000_000,
        "silver" => 0xc0c_0c0,
        "gray" | "grey" => 0x808_080,
        "white" => 0xfff_fff,
        "maroon" => 0x800_000,
        "red" => 0xff0_000,
        "purple" => 0x800_080,
        "fuchsia" => 0xff0_0ff,
        "green" => 0x008_000,
        "lime" => 0x00f_f00,
        "olive" => 0x808_000,
        "yellow" => 0xfff_f00,
        "navy" => 0x000_080,
        "blue" => 0x000_0ff,
        "teal" => 0x008_080,
        "aqua" => 0x00f_fff,
        _ => return None,
    }))
}

fn css_string(name: &str) -> Option<String> {
    Some(String::from(name))
}

fn parse_string<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<String, ParseError<'i, CustomParseError>> {
    Ok(match input.next()? {
        Token::QuotedString(s) => match css_string(&s) {
            Some(string) => string,
            None => return Err(CustomParseError::InvalidStringName(s.into_owned()).into()),
        },

        t => {
            let basic_error = BasicParseError::UnexpectedToken(t);
            return Err(basic_error.into());
        }
    })
}

fn parse_basic_color<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Color, ParseError<'i, CustomParseError>> {
    Ok(match input.next()? {
        Token::Ident(s) => match css_color(&s) {
            Some(color) => color,
            None => return Err(CustomParseError::InvalidColorName(s.into_owned()).into()),
        },

        Token::IDHash(hash) | Token::Hash(hash) => match hash.len() {
            6 | 8 => {
                let mut x = match u32::from_str_radix(&hash, 16) {
                    Ok(x) => x,
                    Err(_) => {
                        return Err(CustomParseError::InvalidColorHex(hash.into_owned()).into());
                    }
                };

                if hash.len() == 6 {
                    x |= 0xFF_000_000;
                }

                Color { data: x }
            }
            _ => return Err(CustomParseError::InvalidColorHex(hash.into_owned()).into()),
        },

        t => {
            let basic_error = BasicParseError::UnexpectedToken(t);
            return Err(basic_error.into());
        }
    })
}

pub fn parse(s: &str) -> Vec<Rule> {
    let mut input = ParserInput::new(s);
    let mut parser = Parser::new(&mut input);
    let rule_parser = RuleParser::new();

    let rules = {
        let rule_list_parser =
            cssparser::RuleListParser::new_for_stylesheet(&mut parser, rule_parser);
        rule_list_parser.collect::<Vec<_>>()
    };

    for rule in &rules {
        match *rule {
            Ok(_) => {}
            Err(ref e) => {
                match e.error {
                    ParseError::Basic(ref e) => eprintln!("{:?}", e),
                    ParseError::Custom(ref e) => eprintln!("{:?}", e),
                }
                println!("Error occured in `{}`", parser.slice(e.span.clone()));
            }
        }
    }

    rules.into_iter().filter_map(|rule| rule.ok()).collect()
}

const fn hex(data: u32) -> Color {
    Color {
        data: 0xFF_000_000 | data,
    }
}
