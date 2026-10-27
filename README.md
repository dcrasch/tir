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
The full list, in registry order:

| id | Tile | Heesch | IH | Group | Colours |
|---|---|---|---|---|---|
| `quad1` | Parallelograms | TTTT | 41 | p1 | 2 |
| `hex1` | Hexagons | TTTTTT | 1 | p1 | 3 |
| `tri2` | Triangles, rotated | CCC | 84 | p2 | 2 |
| `quad2edge` | Parallelograms, rotated | TCTC | 47 | p2 | 2 |
| `quad2tip` | Quadrilaterals, rotated | CCCC | 46 | p2 | 2 |
| `pent2` | Pentagons, rotated | TCTCC | 23 | p2 | 3 |
| `hex2` | Hexagons, rotated | TCCTCC | 4 | p2 | 3 |
| `quad3` | Rhombuses, 3-way rotation | C3C3C3C3 | 33 | p3 | 3 |
| `hex3` | Hexagons, 3-way rotation | C3C3C3C3C3C3 | 7 | p3 | 3 |
| `tri4` | Triangles, 4-way rotation | CC4C4 | 79 | p4 | 2 |
| `quad4` | Squares, 4-way rotation | C4C4C4C4 | 55 | p4 | 2 |
| `pent4` | Pentagons, 4-way rotation | CC4C4C4C4 | 28 | p4 | 3 |
| `tri6alone` | Equilateral triangles, 6-way rotation | CC6C6 | 88 | p6 | 2 |
| `tri6tip` | Isosceles triangles, 6-way rotation | CC3C3 | 39 | p6 | 3 |
| `quad6` | Kites, 6-way rotation | C3C3C6C6 | 31 | p6 | 3 |
| `pent6` | Pentagons, 6-way rotation | CC3C3C6C6 | 21 | p6 | 3 |
| `quad2flipedge` | Parallelograms, flipped | TGTG | 43 | pg | 2 |
| `quad2fliptip` | Kites, flipped | G1G1G2G2 | 44 | pg | 2 |
| `hex2flipend` | Hexagons, flipped (on perpendicular axis) | TG1G2TG2G1 | 3 | pg | 3 |
| `hex2flipsides` | Hexagons, flipped (on parallel axis) | TG1G1TG2G2 | 2 | pg | 3 |
| `tri4flip` | Triangles, rotated and flipped | CGG | 86 | pgg | 2 |
| `quad4fliptip` | Quadrilaterals, touching, rotated and flipped | CCGG | 53 | pgg | 2 |
| `quad4flipalonerotate` | Quadrilaterals, isolated, rotated and flipped | CGCG | 51 | pgg | 2 |
| `quad4flipaloneflip` | Rectangles, 2-way flip | G1G2G1G2 | 52 | pgg | 2 |
| `pent4flipedge` | Pentagons, rotated and flipped | TCTGG | 25 | pgg | 3 |
| `pent4flipalone` | Pentagons, 2-way flip | CG1G2G1G2 | 27 | pgg | 3 |
| `hex4flipedge` | Hexagons, rotated and flipped | TCCTGG | 5 | pgg | 3 |
| `hex4flipalone` | Hexagons, 2-way flip | CG1CG2G1G2 | 6 | pgg | 3 |
| `quad1sym` | Rhombuses, with inner mirror | TT\|TT | 68 | cm | 2 |
| `hex1sym` | Hexagons, with inner mirror | tTTtTT | 12 | cm | 3 |
| `quad2edgesym` | Rectangles, with inner mirror, rotated | tCtC | 66 | pmg | 2 |
| `quad2tipsym` | Kites, with inner mirror, rotated | CC\|CC | 69 | pmg | 2 |
| `hex2sym` | Hexagons, with inner mirror, rotated | tCCtCC | 13 | pmg | 3 |
| `quad3sym` | Rhombuses, with inner mirror and 3-way rotation | C3C3\|C3C3 | 36 | p31m | 3 |
| `quad4sym` | Squares, with inner mirror and 4-way rotation | C4C4\|C4C4 | 71 | p4g | 2 |

In `tir`, press `n` for the next figure and `p` for the previous one. The window
title shows the figure name, Heesch code, isohedral (IH) type and wallpaper
group, e.g.
`Tessellations of Squares, 4-way rotation (C4C4C4C4, IH 55, p4)`.

`tir-web` and `tir-wgpu` use four ceramic colours sampled from the Alhambra
"Brilliant" palette (Amalfi, Capri, Canary and Denim); `tir-wgpu` adds a subtle
screen-space grain. Press `r` in `tir-wgpu` or `c` in `tir-web` to cycle the
colour assignment. The web version also has a **Colours** menu with the classic
red/green/blue/black colours and a grayscale palette.

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

Tessellation in Rust Copyright (c) 2022-2026 David Rasch
