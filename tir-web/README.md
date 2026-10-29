# Tessellation Web

## Live version

https://dcrasch.github.io/tir/

## Usage

The **Tile** menu lists all 35 symmetries grouped by tile shape, e.g.
`Triangles` with `rotated`, `4-way rotation` and `rotated and flipped` beneath
it. Shapes are ordered by their lowest isohedral (IH) type and the variants
within a shape by IH, so the list reads roughly by IH number. Each entry names
the shape and its IH type as well as the variant — `Pentagons, rotated — TCTCC,
IH 23, p2` — because a closed menu shows only the selected entry, never the
shape heading. The window title shows the same details. Press `n` for the next
figure and `p` for the previous one, or click a handle in the drawing and drag
it to reshape the tile.

The **Colours** menu below it switches the tile palette: **Alhambra** (the
default, four ceramic colours from the Alhambra "Brilliant" palette),
**Classic** (red, green, blue and black) and **Grayscale**. Press `c` to cycle
the palettes.

## Build the project

### Requirements

* rust
* trunk https://trunkrs.dev/

### Install rust and wasm with rustup

1. Goto https://rustup.rs/
2. Follow the instruction to install rustup and rust
3. Add rust for wasm:
```
rustup target add wasm32-unknown-unknown
```

### Install trunk

```
cargo install trunk
```

## deployment

Use `--public-url` to set the base url.
```
trunk build --release
scp ../docs/* you@example.com:/var/www
```

or copy it to your docs directory and use github.io

## development

Start dev server and opens a browser.
```
trunk serve --release --open
```

## Notes

* Set mode: 'production' or it will be very slow
* Doesn't work in old Edge browsers (12-18) https://caniuse.com/mdn-api_textencoder_encoding

## Links

* https://rustup.rs/
* https://trunkrs.dev/
* https://rustwasm.github.io/docs/wasm-bindgen/
* https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
* https://github.com/rustwasm/wasm-bindgen/tree/master/examples/paint
* https://gtk-rs.org/docs-src/tutorial/closures
* https://github.com/sn99/wasm-template-rust
* https://rustwasm.github.io/book/reference/deploying-to-production.html#ensure-that-your-http-server-uses-the-applicationwasm-mime-type

## Copyright

Tessellation in Rust Copyright (c) 2022-2026 David Rasch