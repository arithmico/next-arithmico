# cssbundler

`cssbundler` is a CLI tool written in Rust that finds all CSS files matching a given glob pattern and bundles them into a single CSS file. It is useful for optimizing stylesheets in web projects.

## Features

- Finds CSS files using a glob pattern.
- Merges multiple CSS files into one.

## Installation

You can install `cssbundler` using Cargo:

```sh
cargo install cssbundler
```

## Usage

Run the tool with a glob pattern and specify an output file:

```sh
cssbundler -i "src/styles/**/*.css" -o dist/bundle.css
```

### Example

Given the following structure:

```
src/styles/
  ├── base.css
  ├── layout.css
  ├── theme.css
```

Running:

```sh
cssbundler "src/styles/*.css" -o dist/bundle.css
```

Produces `dist/bundle.css` containing the merged contents of all matched files.

## Technologies Used

- **Programming Language:** Rust
- **File Matching:** Glob Patterns
