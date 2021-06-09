# Material design icons, updated font

Material Design Icons is the official [icon set](https://www.google.com/design/spec/style/icons.html#icons-system-icons) from Google. The icons are designed under the [material design guidelines](https://material.io/guidelines/).

## Updated set

This is an updated version of icons, which includes all icons available at [material.io](https://material.io/resources/icons/).

You can find an older version of this icon set in [google/material-design-icons](https://github.com/google/material-design-icons) repository.

Because the official repository is no longer maintained, I have decided to make an alternative repository with the latest icons.

## Available icons

Version 3 that is available in the official icons repository only includes 1 variation of each icon.

This repository includes several variations for each icon:

- baseline
- sharp
- outline
- round
- two-tone

This repository includes only icons as icon font. Other available formats are in different repositories:

- SVG: [material-icons](https://github.com/material-icons/material-icons)
- PNG: [material-icons-png](https://github.com/material-icons/material-icons-png)

If you need another format, please open an issue on [material icons repository](https://github.com/material-icons/material-icons) and specify what format, size and colour you need.

Icons are also available with Iconify SVG framework. See below.

## Getting started

This readme explains how to use icon font in your projects.

To get the icon font, you can either [clone GitHub repository](https://github.com/material-icons/material-icons-font) or install `@material-icons/font` NPM package.

### Include stylesheet

To use icons in your project you need to include stylesheet.

Add the following code to your page:

```html
<link
	rel="stylesheet"
	href="https://material-icons.github.io/material-icons-font/css/all.css"
/>
```

This will include all 5 fonts and all icons. If you want to include only 1 type of font, change "all.css" to one of the following:

- baseline.css for base font
- outline.css for "MaterialIcons Outline"
- round.css for "MaterialIcons Round"
- sharp.css for "MaterialIcons Sharp"
- twotone.css for "MaterialIcons TwoTone"

### No ligatures

The official font does not work in older browsers because of the way it handles two-tone icons. It renders both opaque and transparent parts of the icon in the same glyph, which is not supported by older browsers.

Another downside of ligatures is an issue with search engines and screen readers. Search engines index icon name as text, which can ruin search engine optimisation. This icon font uses Unicode characters from sections that are not used in Unicode specification and are available for custom use, therefore screen readers and search engines simply ignore characters used in icons.

Unlike the official font from Google, this font does not use ligatures. Implementing two-tone icons while supporting older browsers is not possible with ligatures. A downside is this font has quite a big stylesheet.

### HTML syntax

To use any icon, add "material-icon" to list of classes and icon name with "md-" prefix:

```html
<i class="material-icons md-email"></i>
```

This will display icon with base font, 24px height.

To change font size you can use css or add one of predefined classes:

- "md-18" for 18px height
- "md-24" for 24px height (default)
- "md-36" for 36px height
- "md-48" for 48px height

```html
<i class="material-icons md-48 md-signal_wifi_3_bar"></i>
```

To change colour change text colour. There are also several predefined colours:

- "md-dark" for the black icon (on light background)
- "md-light" for the white icon (on dark background)

Additionally to those colors you can set md-inactive for grey inactive color:

```html
<i class="material-icons md-dark md-turned_in"></i>
<i class="material-icons md-dark md-inactive md-turned_in_not"></i>
```

If you are using all.css, to use different icon font use class name for that icon font instead of material-icons:

- "material-icons-outline" for "MaterialIcons Outline" font
- "material-icons-round" for "MaterialIcons Round" font
- "material-icons-sharp" for "MaterialIcons Sharp" font
- "material-icons-twotone" for "MaterialIcons TwoTone" font

```html
<i class="material-icons-sharp md-shuffle"></i>
<i class="material-icons-twotone md-videocam_off"></i>
```

### Icon names

Icon name is same as the ligature in the official font, but with "md-" prefix.

Why "md-" prefix? Because some icon names start with numbers, such as "3d_rotation". "3d_rotation" is not a valid class name, so prefix had to be added. After prefix icon name is the same as in ligature in the official font.

Examples:

- md-poll
- md-print_disabled
- md-signal_cellular_connected_no_internet_3_bar

### Two-tone icons

Two-tone icons are implemented by splitting icon into two parts: opaque part and transparent part, then displaying them as 2 separate glyphs placed on top of each other.

For example, this is what "battery_30" icon looks like: ![baseline-battery_30.svg](https://material-icons.github.io/material-icons-font/samples/baseline-battery_30.svg)

Font uses 2 glyphs to represent that icon:

- transparent part ![baseline-battery_30-transparent.svg](https://material-icons.github.io/material-icons-font/samples/baseline-battery_30-transparent.svg)
- opaque part ![baseline-battery_30-opaque.svg](https://material-icons.github.io/material-icons-font/samples/baseline-battery_30-opaque.svg)

They are layered on top of each other. ":before" is used to display transparent part with 30% opacity, ":after" is used to display opaque part.

#### Changing colours in two-tone icons

Using stylesheet you can change the colour of any part.

To change color for opaque part, change color of :after pseudo element:

```css
/* for all icons */
.md:after {
	color: red;
}
/* for specific icon */
.md-view_carousel:after {
	color: purple;
}
/* or custom class added to specific icon */
.icon-green-red:after {
	color: green;
}
```

To change color for transparent part, change color (and opacity) of :before pseudo element:

```css
/* for all icons */
.md:before {
	color: blue;
	opacity: 1;
}
/* for specific icon */
.md-view_carousel:after {
	color: lightblue;
	opacity: 1;
}
/* or custom class added to specific icon */
.icon-green-red:after {
	color: red;
	opacity: 1;
}
```

## Avoid using icon fonts!

More and more icon fonts are moving away from fonts to various SVG frameworks.

Several years ago when icon fonts became popular browsers had poor support for SVG and JavaScript was slow, therefore font was a better solution.

There are many downsides to using fonts that cannot be addressed, but support for SVG and SVG frameworks have improved.

Font loads all icons that take a while to load and quite a big stylesheet. Also, fonts render with blurred ugly edges on some operating systems, worst offender is Windows.

Good news, all icons are available as modern JavaScript framework that replaces glyph fonts. See below.

### JavaScript framework

All icons are available with [Iconify JavaScript framework](https://iconify.design/).

#### What is Iconify?

Iconify project makes it easy to add SVG icons to websites and offers over 40,000 icons to choose from.

You can use Iconify not only with this icon set, but also [Templarian's Material Design Icons](https://iconify.design/icon-sets/mdi/), [Material Design Light](https://iconify.design/icon-sets/mdi-light/), [FontAwesome 5](https://iconify.design/icon-sets/fa-regular/) and many other icon sets on the same page without loading massive fonts.

How is it achieved? Iconify project uses a new innovative approach to loading icons. Unlike fonts and SVG frameworks, Iconify only loads icons that are used on the current page instead of loading entire fonts. How is it done? By serving icons dynamically from publicly available JSON API (you can make a copy of script and API if you prefer to keep everything on your servers).

Iconify is designed to be as easy to use as possible. It uses icon placeholder syntax and icons inherit size and colour from the parent element, so they are easy to style with CSS.

#### How to use this icon set with Iconify?

Add this line to your page to load Iconify:

```
<script src="https://code.iconify.design/1/1.0.4/iconify.min.js"></script>
```

you can add it to `<head>` section of the page or before `</body>`.

To add any icon, write something like this:

```
<span class="iconify" data-icon="ic:baseline-access-time" data-inline="false"></span>
```

or this:

```
<iconify-icon data-icon="ic:twotone-account-circle"></iconify-icon>
```

There are 2 small differences in icon names when using Iconify: icons have "ic:" prefix and underscores in icon name have been replaced by the dash.

See [how to use Iconify](https://iconify.design/docs/iconify-in-pages/) tutorial and [browse MDI icons list](https://iconify.design/icon-sets/ic/) to get code for each icon.

#### Stylesheet

Iconify is not specific to this icon set. It does not force font size, so by default icon size is set to 1em and classes like .md-18 are not supported. You will need to add custom CSS to change the font size.

### React components

No need to reinvent components. See [Iconify for React](https://github.com/iconify/iconify-react).

How to use it:

```
npm install @iconify/react @iconify/icons-ic
```

```js
import { Icon, InlineIcon } from '@iconify/react';
import face from '@iconify/icons-ic/baseline-face';
import home from '@iconify/icons-ic/twotone-home';
```

```jsx
<Icon icon={home} />
<p>This is some text with <InlineIcon icon={face} /></p>
```

## License

(copied from Google's repository)

We have made these icons available for you to incorporate into your products under the [Apache License Version 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt). Feel free to remix and re-share these icons and documentation in your products.
We'd love attribution in your app's _about_ screen, but it's not required. The only thing we ask is that you not re-sell these icons.
