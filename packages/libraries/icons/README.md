# Icons

This package provides Yew components for various icons.

## Development

To start developing with these icons, you need to set up a Yew project.

## Installation

Add the icons package to your `Cargo.toml`:
```toml
[dependencies]
icons = { path = "./relative/path/to/this/package" }
```

## Usage

You can use the icons provided by this package in your Yew application. 
Each icon sets `aria-hidden="true"` for accessibility and can be styled using the `class` property. Here’s a simple example:

```rust
use icons::expand::ExpandIcon;

fn view() -> Html {
    html! {
        <ExpandIcon class="my-icon-class" />
    }
}
```

## Accessibility

All icons in this package include `aria-hidden="true"` to ensure they are ignored by screen readers, making them purely decorative.

## Styling

Icons can be styled using the `class` property. 
Apply your custom styles by defining CSS classes and assigning them to the icons:

```css
/* styles.css */
.my-icon-class {
    fill: red;
    width: 24px;
    height: 24px;
}
```

```rust
use icons::expand::ExpandIcon;

fn view() -> Html {
    html! {
        <ExpandIcon class="my-icon-class" />
    }
}
```
