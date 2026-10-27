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
title shows the figure name and Heesch code, e.g.
`Tessellations of Squares with 4-way rotation (C4C4C4C4)`.

Every figure is checked by `tessellations/tests/test_grid.rs`, which verifies
that the generated tiling covers a sampled area exactly once (no gaps or
overlaps). Base polygons and edge pairings for the IH types follow
[tactile-rs](https://github.com/abusch/tactile-rs/), and each figure is coloured
with tactile's per-type colouring table (`coloring.rs`), giving the same balanced
colour patterns as tactile.

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
