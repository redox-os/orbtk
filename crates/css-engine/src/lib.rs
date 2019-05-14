/*!

This crate provides functions to load a css files as theme and access it properties with
selectors. 

This crate depends on the orbtk_utils crate.

# Example

Basic usage of the tree:

```rust,no_run

use orbtk_css_engine::prelude::*;

let mut theme = Theme::create_from_path("theme.css").build();
let selector = Selector::from("button");
let background = theme.brush("background", &selector);

```

 */

pub use selector::*;
pub use theme::*;

pub mod prelude;
mod selector;
mod theme;
