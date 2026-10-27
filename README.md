# tir

Tessellation In Rust.

![tessellations-minifb.png](https://github.com/dcrasch/tir/blob/master/tessellations-minifb.png?raw=true)

Live view : https://dcrasch.github.io/tir/

## Tessellation symmetries

All 35 symmetries from
[tiled.art's summary](https://tiled.art/en/symmetrySummary/) are supported
(28 Heesch types plus 7 types with an inner mirror). The figures live in
`tessellations/src/tessellationfigure/`; `registry.rs` holds the id, name,
Heesch code, isohedral type (IH), wallpaper group and colour count of each one.

In `tir`, press `n` for the next figure and `p` for the previous one. The window
title shows the figure name, Heesch code, isohedral (IH) type and wallpaper
group, e.g.
`Tessellations of Squares with 4-way rotation (C4C4C4C4, IH 55, p4)`.

`tir-wgpu` uses four ceramic colours sampled from the Alhambra "Brilliant"
palette (Amalfi, Capri, Canary and Denim) and a subtle screen-space grain.
Press `r` there to cycle the colour assignment.

Every figure is checked by `tessellations/tests/test_grid.rs`, which verifies
that the generated tiling covers a sampled area exactly once (no gaps or
overlaps). Base polygons and edge pairings for the IH types follow
[tactile-rs](https://github.com/abusch/tactile-rs/), and each figure is coloured
with tactile's per-type colouring table (`coloring.rs`), giving the same balanced
colour patterns as tactile.

## Figure JSON

`tir` and `tir-wgpu` save the current figure to `figure.json` with `s` and load
it again with `l`. The file is the plain serde representation of
`TessellationFigure`. There is no version field and no migration: the old
format is not supported. Unknown fields are rejected, so a file containing the
old `ci`, `angle`, `tx`, `ty`, `shape`, `grid` or `rotdiv` fields will not load.

A minimal complete figure (the IH 41 square) looks like this:

```json
{
  "lines": [
    {
      "points": [{"x": 0.0, "y": 0.0}, {"x": 1.0, "y": 0.0}],
      "transform": [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
    },
    {
      "points": [{"x": 1.0, "y": 0.0}, {"x": 1.0, "y": 1.0}],
      "transform": [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
    },
    {
      "points": [{"x": 1.0, "y": 1.0}, {"x": 0.0, "y": 1.0}],
      "transform": [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
    },
    {
      "points": [{"x": 0.0, "y": 1.0}, {"x": 0.0, "y": 0.0}],
      "transform": [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
    }
  ],
  "links": [
    {"a": 0, "b": 2, "symmetry": false, "transform": [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]},
    {"a": 1, "b": 3, "symmetry": false, "transform": [1.0, 0.0, 0.0, 1.0, -1.0, 0.0]}
  ],
  "colors": 2,
  "ih": 41
}
```

- `lines` is the tile outline in order. Edge `i` runs from the first to the last
  point of its `points` array; consecutive edges share endpoints. Intermediate
  points are editable handles and are copied to linked edges.
- A line's own `transform` is normally the identity. The edge-to-edge
  isometries used to build the tiling live in `links`.
- `links` connects edge `a` to edge `b`. `transform` maps `a` onto `b` with the
  direction reversed; `symmetry: true` marks an inner mirror, which keeps the
  tile symmetric but does not create a neighbouring tile.
- The six transform values are `[m11, m12, m21, m22, m31, m32]`, the same order
  as an SVG `matrix(...)`: `x' = m11*x + m21*y + m31`, `y' = m12*x + m22*y + m32`.
- `colors` is the number of colours in the tactile table. `ih` is the tactile
  isohedral type (1–93), or `null` for a figure that is not one of the built-in
  symmetries (for example a figure built with `TessellationFigure::new()`).

## TODO

- [x] Build simple ui with minifb
- [x] Build web ui with webassembly 
- [x] Publish tessellations crate on crates.io
- [ ] Build flutter? android ios mobile app using rid
- [ ] Write a blog about building the program
- [ ] Write more documentation about tiny-skia and euclid.

## Rust programming docs and example code

* https://doc.rust-lang.org/book/ch11-03-test-organization.html
* https://github.com/redox-os/orbtk/blob/develop/crates/render/src/web/mod.rs
* https://github.com/linebender/tiny-skia/tree/main/examples
* https://github.com/serde-rs/serde/issues/723

## Links

* https://github.com/isohedral/tactile-js
* https://github.com/abusch/tactile-rs
* https://tiled.art/en/symmetrySummary/
* https://github.com/DBraun/tactile-python
* https://github.com/linebender/tiny-skia
* https://github.com/nical/lyon
* https://github.com/gfx-rs/wgpu
* https://github.com/flutter-rs/flutter-rs
* https://github.com/brickpop/flutter-rust-ffi
* https://thlorenz.com/rid-site/docs/getting-started/introduction/
* https://crates.io/crates/tactile
* https://crates.io/crates/tiling

## License

This code is distributed under a GPL v3 style license, see the LICENSE file for complete information.

## Copyright

Tessellation in Rust Copyright (c) 2022 David Rasch
