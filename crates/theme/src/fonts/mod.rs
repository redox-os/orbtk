/// The 'Material Design Icons' is the official icon set from Google.
/// The icons are designed under the material design guidelines.
/// Because the official repository is no longer maintained from Google,
/// source code is now maintained and hosted at:
/// https://github.com/material-icons/material-icons-font
/// Package-Info: package.json

// TL;DR
// https://stackoverflow.com/questions/11002820/why-should-we-include-ttf-eot-woff-svg-in-a-font-face
// ... woff2 gets drafted and accepted, which improves the compression
// leading to even smaller files, along with the ability to load a single font
// "in parts" so that a font that supports 20 scripts can be stored as "chunks"
// on disk instead, with browsers automatically able to load the font "in parts"
// as needed, rather than needing to transfer the entire font up front, further
// improving the typesetting experience.
// If you don't want to support IE 8 and lower, and iOS 4 and lower,
// and android 4.3 or earlier, then you can just use WOFF
// (and WOFF2, a more highly compressed WOFF, for the newest browsers that support it.)

// Reference to the material icon fonts.
pub const MATERIAL_ICONS_FONT_TTF: &[u8] = include_bytes!("MaterialIcons.ttf");

pub const MATERIAL_ICONS_BASELINE_FONT: &[u8] = include_bytes!("MaterialIcons-Baseline.woff2");
pub const MATERIAL_ICONS_OUTLINED_FONT: &[u8] = include_bytes!("MaterialIcons-Outlined.woff2");
pub const MATERIAL_ICONS_ROUND_FONT: &[u8] = include_bytes!("MaterialIcons-Round.woff2");
pub const MATERIAL_ICONS_SHARP_FONT: &[u8] = include_bytes!("MaterialIcons-Sharp.woff2");
pub const MATERIAL_ICONS_TWOTONE_FONT: &[u8] = include_bytes!("MaterialIcons-TwoTone.woff2");

// Reference to the roboto fonts.
pub const ROBOTO_REGULAR_FONT: &[u8] = include_bytes!("Roboto-Regular.ttf");
pub const ROBOTO_MEDIUM_FONT: &[u8] = include_bytes!("Roboto-Medium.ttf");

// font sizes
pub const FONT_SIZE_12: f64 = 12.0;

// icon sizes
pub const ICON_FONT_SIZE_12: f64 = 12.0;
