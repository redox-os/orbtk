use cssparser::{self, BasicParseError, CompactCowStr, DeclarationListParser, Parser, ParseError, ParserInput, Token};
use orbclient::Color;
use std::collections::HashSet;
use std::sync::Arc;
use std::mem;
use std::ops::Add;

static DEFAULT_THEME_CSS: &'static str = include_str!("theme.css");

lazy_static! {
    static ref DEFAULT_THEME: Arc<Theme> = {
        Arc::new(Theme {parent: None, rules: parse(DEFAULT_THEME_CSS)})
    };
}

pub struct Theme {
    parent: Option<Arc<Theme>>,
    rules: Vec<Rule>,
}

impl Theme {
    pub fn new() -> Self {
        Theme::parse("")
    }

    pub fn parse(s: &str) -> Self {
        Theme {
            parent: Some(DEFAULT_THEME.clone()),
            rules: parse(s),
        }
    }

    fn all_rules(&self) -> Vec<Rule> {
        if let Some(ref parent) = self.parent {
            parent.rules.iter().chain(self.rules.iter()).cloned().collect()
        } else {
            self.rules.clone()
        }
    }

    pub fn get(&self, property: &str, query: &Selector) -> Option<Value> {
        let mut matches: Vec<(bool, Specificity, Value)> = Vec::new();

        for rule in self.all_rules().iter().rev() {
            let matching_selectors = rule.selectors.iter().filter(|x| x.matches(query)).collect::<Vec<_>>();

            if matching_selectors.len() > 0 {
                if let Some(decl) = rule.declarations.iter().find(|decl| decl.property == property) {
                    let highest_specifity = matching_selectors.iter().map(|sel| sel.specificity()).max().unwrap();
                    matches.push((decl.important, highest_specifity, decl.value.clone()));
                }
            }
        }

        matches.sort_by_key(|x| (x.0, x.1));
        matches.last().map(|x| x.2.clone())
    }

    pub fn color(&self, property: &str, query: &Selector) -> Color {
        let default = Color { data: 0 };
        self.get(property, query).map(|v| v.color().unwrap_or(default)).unwrap_or(default)
    }

    pub fn uint(&self, property: &str, query: &Selector) -> u32 {
        self.get(property, query).map(|v| v.uint().unwrap_or(0)).unwrap_or(0)
    }
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Clone, Debug)]
pub enum SelectorRelation {
    Ancestor(Selector),
    Parent(Selector),
}

impl<T: Into<String>> From<T> for Selector {
    fn from(t: T) -> Self {
        Selector::new(Some(t.into()))
    }
}

/// Describes the specificity of a selector.
///
/// The indexes are as follows:
/// 0 - number of IDs (most important)
/// 1 - number of classes and pseudo-classes
/// 2 - number of elements (least important)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Specificity([u8; 3]);

impl Add<Self> for Specificity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Specificity([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

#[derive(Clone, Debug, Default)]
pub struct Selector {
    pub element: Option<String>,
    pub classes: HashSet<String>,
    pub pseudo_classes: HashSet<String>,
    pub relation: Option<Box<SelectorRelation>>,
}

impl Selector {
    pub fn new<S: Into<String>>(element: Option<S>) -> Self {
        Selector {
            element: element.map(|s| s.into()),
            classes: HashSet::new(),
            pseudo_classes: HashSet::new(),
            relation: None,
        }
    }

    fn specificity(&self) -> Specificity {
        let s = Specificity([
            0,
            (self.classes.len() + self.pseudo_classes.len()) as u8,
            if self.element.is_some() { 1 } else { 0 }
        ]);

        if let Some(ref relation) = self.relation {
            match **relation {
                SelectorRelation::Ancestor(ref x) | SelectorRelation::Parent(ref x) => return x.specificity() + s,
            }
        }

        s
    }

    pub fn matches(&self, other: &Selector) -> bool {
        if self.element.is_some() && self.element != other.element {
            return false;
        }

        if !other.classes.is_superset(&self.classes) {
            return false;
        }

        if !other.pseudo_classes.is_superset(&self.pseudo_classes) {
            return false;
        }

        true
    }

    pub fn with_class<S: Into<String>>(mut self, class: S) -> Self {
        self.classes.insert(class.into());
        self
    }

    pub fn with_pseudo_class<S: Into<String>>(mut self, pseudo_class: S) -> Self {
        self.pseudo_classes.insert(pseudo_class.into());
        self
    }
}

impl Selector {
    pub fn is_empty(&self) -> bool {
        self.element.is_none() && self.classes.is_empty() && self.pseudo_classes.is_empty()
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
    Color(Color),
}

impl Value {
    pub fn uint(&self) -> Option<u32> {
        match *self {
            Value::UInt(x) => Some(x),
            _ => None,
        }
    }

    pub fn color(&self) -> Option<Color> {
        match *self {
            Value::Color(x) => Some(x),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum CustomParseError {
    InvalidColorName(String),
    InvalidColorHex(String),
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

    fn parse_prelude<'t>(&mut self, input: &mut Parser<'i, 't>)
        -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let res = parse_selectors(input)?;
        Ok(res)
    }

    fn parse_block<'t>(&mut self, selectors: Self::Prelude, input: &mut Parser<'i, 't>)
        -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let decl_parser = DeclarationParser {};

        let decls = DeclarationListParser::new(input, decl_parser).collect::<Vec<_>>();

        for decl in &decls {
            match *decl {
                Ok(_) => {},
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

fn parse_selectors<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Vec<Selector>, ParseError<'i, CustomParseError>> {
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
                    let mut old_selector = Selector::new(Some(element_name.to_string()));
                    mem::swap(&mut old_selector, &mut selector);
                    selector.relation = Some(Box::new(SelectorRelation::Ancestor(old_selector)));
                }
            }

            Token::Delim('>') => {
                let mut old_selector = Selector::new(Some(input.expect_ident()?.to_string()));
                mem::swap(&mut old_selector, &mut selector);
                selector.relation = Some(Box::new(SelectorRelation::Parent(old_selector)));
            }

            // Any element
            Token::Delim('*') => {}

            // Class
            Token::Delim('.') => {selector.classes.insert(input.expect_ident()?.into_owned());}

            // Pseudo-class
            Token::Colon => {selector.pseudo_classes.insert(input.expect_ident()?.into_owned());}

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

    fn parse_value<'t>(&mut self, name: CompactCowStr<'i>, input: &mut Parser<'i, 't>) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let value = match &*name {
            "color" | "border-color" => Value::Color(parse_basic_color(input)?),

            "background" | "foreground" => Value::Color(parse_basic_color(input)?),

            "border-radius" | "border-thickness" => {
                match input.next()? {
                    Token::Number { int_value: Some(x), has_sign, .. } if !has_sign && x >= 0 => Value::UInt(x as u32),
                    t => return Err(BasicParseError::UnexpectedToken(t).into())
                }
            }

            _ => return Err(BasicParseError::UnexpectedToken(input.next()?).into()),
        };

        Ok(Declaration {
            property: name.into_owned(),
            value: value,
            important: input.try(cssparser::parse_important).is_ok()
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

        "black" => 0x000000,
        "silver" => 0xc0c0c0,
        "gray" | "grey" => 0x808080,
        "white" => 0xffffff,
        "maroon" => 0x800000,
        "red" => 0xff0000,
        "purple" => 0x800080,
        "fuchsia" => 0xff00ff,
        "green" => 0x008000,
        "lime" => 0x00ff00,
        "olive" => 0x808000,
        "yellow" => 0xffff00,
        "navy" => 0x000080,
        "blue" => 0x0000ff,
        "teal" => 0x008080,
        "aqua" => 0x00ffff,
        _ => return None,
    }))
}

fn parse_basic_color<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Color, ParseError<'i, CustomParseError>> {
    Ok(match input.next()? {
        Token::Ident(s) => match css_color(&s) {
            Some(color) => color,
            None => return Err(CustomParseError::InvalidColorName(s.into_owned()).into()),
        },

        Token::IDHash(hash) | Token::Hash(hash) => {
            match hash.len() {
                6 | 8 => {
                    let mut x = match u32::from_str_radix(&hash, 16) {
                        Ok(x) => x,
                        Err(_) => return Err(CustomParseError::InvalidColorHex(hash.into_owned()).into()),
                    };

                    if hash.len() == 6 {
                        x |= 0xFF000000;
                    }

                    Color { data: x }
                },
                _ => return Err(CustomParseError::InvalidColorHex(hash.into_owned()).into()),
            }
        }

        t => {
            let basic_error = BasicParseError::UnexpectedToken(t);
            return Err(basic_error.into());
        }
    })
}




fn parse(s: &str) -> Vec<Rule> {
    let mut input = ParserInput::new(s);
    let mut parser = Parser::new(&mut input);
    let rule_parser = RuleParser::new();

    let rules = {
        let rule_list_parser = cssparser::RuleListParser::new_for_stylesheet(&mut parser, rule_parser);
        rule_list_parser.collect::<Vec<_>>()
    };

    for rule in &rules {
        match *rule {
            Ok(_) => {},
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
    Color { data: 0xFF000000 | data }
}
