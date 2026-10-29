# Tessellation Web

## Live version

https://dcrasch.github.io/tir/

## Usage

The app opens on a **gallery** of all 35 symmetries. They are grouped into the
tiled.art chart categories — **1-way**, **2-way**, **2-way with flip**,
**3-way**, **4-way**, **4-way with flip** and **6-way** — and each card shows a
thumbnail of the tiling, the tile name, its Heesch type, IH number and
wallpaper group. Click a card to open the editor on that figure.

The gallery and the editor are two routes of the same page, selected by the URL
fragment: `#/` is the gallery and `#/edit/<id>` is the editor (for example
`#/edit/quad4`). The browser back and forward buttons move between them, and
the editor's **&larr; All tessellations** link returns to the gallery.

In the editor, the figure is the one opened from the gallery. Press `n` for the
next figure and `p` for the previous one (the window title and the URL fragment
follow), or click a handle in the drawing and drag it to reshape the tile.

The **Colours** menu switches the tile palette: **Alhambra** (the default, four
ceramic colours from the Alhambra "Brilliant" palette), **Classic** (red,
green, blue and black) and **Grayscale**. Press `c` to cycle the palettes.

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