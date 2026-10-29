# Milestones: all 35 tiled.art tessellation symmetries

Goal: support every tessellation symmetry listed on
[tiled.art – Summary of Tessellation Symmetries](https://tiled.art/en/symmetrySummary/)
(28 Heesch types + 7 types with an inner mirror) as a `TessellationFigure`.

Status legend: ✅ done · ⬜ todo

## Current state

All 35 symmetries are implemented in
`tessellations/src/tessellationfigure/registry.rs` (`FIGURE_IDS` / `FIGURES`) and
`tiles.rs`/`spec.rs` (constructors). Every figure passes the coverage test in
`tessellations/tests/test_grid.rs`.

| tiled.art id | Symmetry | Heesch | IH | Group |
|---|---|---|---|---|
| `quad1` | Parallelograms | TTTT | 41 | p1 |
| `hex1` | Hexagons | TTTTTT | 1 | p1 |
| `tri2` | Triangles, rotated | CCC | 84 | p2 |
| `quad2edge` | Parallelograms, rotated | TCTC | 47 | p2 |
| `quad2tip` | Quadrilaterals, rotated | CCCC | 46 | p2 |
| `pent2` | Pentagons, rotated | TCTCC | 23 | p2 |
| `hex2` | Hexagons, rotated | TCCTCC | 4 | p2 |
| `quad3` | Rhombuses with 3-way rotation | C3C3C3C3 | 33 | p3 |
| `hex3` | Hexagons with 3-way rotation | C3C3C3C3C3C3 | 7 | p3 |
| `tri4` | Triangles with 4-way rotation | CC4C4 | 79 | p4 |
| `quad4` | Squares with 4-way rotation | C4C4C4C4 | 55 | p4 |
| `pent4` | Pentagons with 4-way rotation | CC4C4C4C4 | 28 | p4 |
| `tri6alone` | Equilateral triangles with 6-way rotation | CC6C6 | 88 | p6 |
| `tri6tip` | Isosceles triangles with 6-way rotation | CC3C3 | 39 | p6 |
| `quad6` | Kites with 6-way rotation | C3C3C6C6 | 31 | p6 |
| `pent6` | Pentagons with 6-way rotation | CC3C3C6C6 | 21 | p6 |
| `quad2flipedge` | Parallelograms, flipped | TGTG | 43 | pg |
| `quad2fliptip` | Kites, flipped | G1G1G2G2 | 44 | pg |
| `hex2flipend` | Hexagons, flipped (perpendicular axis) | TG1G2TG2G1 | 3 | pg |
| `hex2flipsides` | Hexagons, flipped (parallel axis) | TG1G1TG2G2 | 2 | pg |
| `tri4flip` | Triangles, rotated and flipped | CGG | 86 | pgg |
| `quad4fliptip` | Quadrilaterals (touching), rotated and flipped | CCGG | 53 | pgg |
| `quad4flipalonerotate` | Quadrilaterals (isolated), rotated and flipped | CGCG | 51 | pgg |
| `quad4flipaloneflip` | Rectangles with 2-way flip | G1G2G1G2 | 52 | pgg |
| `pent4flipedge` | Pentagons, rotated and flipped | TCTGG | 25 | pgg |
| `pent4flipalone` | Pentagons with 2-way flip | CG1G2G1G2 | 27 | pgg |
| `hex4flipedge` | Hexagons, rotated and flipped | TCCTGG | 5 | pgg |
| `hex4flipalone` | Hexagons with 2-way flip | CG1CG2G1G2 | 6 | pgg |
| `quad1sym` | Rhombuses, with inner mirror | TT\|TT | 68 | cm |
| `hex1sym` | Hexagons, with inner mirror | tTTtTT | 12 | cm |
| `quad2edgesym` | Rectangles (with inner mirror), rotated | tCtC | 66 | pmg |
| `quad2tipsym` | Kites (with inner mirror), rotated | CC\|CC | 69 | pmg |
| `hex2sym` | Hexagons (with inner mirror), rotated | tCCtCC | 13 | pmg |
| `quad3sym` | Rhombuses, with inner mirror and 3-way rotation | C3C3\|C3C3 | 36 | p31m |
| `quad4sym` | Squares, with inner mirror and 4-way rotation | C4C4\|C4C4 | 71 | p4g |

The 35 symmetries in the table above are all reachable from the app's figure
registry. `tir`, `tir-web` and `tir-wgpu` start on the square and cycle through
all 35 with `n`/`p`; the title shows
`Tessellations of {name} ({Heesch}, IH {ih}, {group})`. The old standalone
`brick()` and `TessellationPlane` helpers have been removed.

`tir-web` opens on a gallery of all 35 figures grouped by the tiled.art chart
categories (`figure_categories()`), and a card click starts the editor on that
figure. The editor has a **Colours** menu for the Alhambra, Classic and
Grayscale palettes (`c` cycles them) and cycles figures with `n`/`p`. The
palettes live in `tessellations/src/render/palette.rs` and are shared with
`tir-wgpu`.

`tir-wgpu` uses the Alhambra palette (Amalfi, Capri, Canary and Denim), applies
a subtle screen-space grain, and cycles the colour assignment with `r`. Its
primitive buffer holds the densest tiling (`tri6tip`, 2573 placements), and
back-face culling is disabled because reflected placements reverse triangle
winding.

### What changed from the original model

- `TessellationLine` edges plus `EdgeLink`s (one isometry per paired edge) define
  the tile; placements are found by walking the links (BFS), including reflections
  and glide reflections.
- `TessellationFigure::placements()` replaces the old
  `TessellationPlane::grid`/`rotdiv` layout and drives every renderer.
- Renderers consume full 2×3 transforms (mirrored copies included); tiles are
  coloured with the per-type colouring tables from
  [tactile-rs](https://github.com/abusch/tactile-rs/), so each figure uses the same
  balanced colour patterns as tactile (or a lattice-parity fallback for figures
  without an IH type).
- Figures can carry inner mirror axes (`from_polygon_with_mirrors`), which are
  propagated to edited points but never create extra tiles.
- `FigureInfo` separates the tile `shape` from the `variant`, and
  `figure_groups()` groups the figures by shape (each group and its variants
  ordered by IH) for a grouped tile picker.
- Render palettes are shared data (`render/palette.rs`): `ALHAMBRA`, `CLASSIC`,
  `GRAYSCALE` and `PALETTES`, with `DEFAULT_PALETTE` set to Alhambra.
- `tir-wgpu` renders both tile orientations: reflections have a negative
  determinant and would otherwise be culled as back faces. The primitive buffer
  is large enough for the densest tiling, and the shader uses the Alhambra
  palette plus a subtle grain.

### Verification

Latest verification (the tile picker and palette work in the working tree):

- `cargo test --workspace` (35 core, 11 grid, 11 line, 1 `tir-wgpu` test)
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace`
- `cargo check -p tir-web --target wasm32-unknown-unknown`
- `cargo fmt --all -- --check`

---

## Milestone 0 – Generalise the core model (no new tiles)

Refactor so that every later milestone is "add a figure definition" instead of
"change the engine". The 6 existing figures must keep working and the existing tests
must pass.

- [x] **Edge model**: describe the tile outline as an ordered list of edges
      (Heesch order, clockwise). Each edge is either
      - a *source* edge with editable points, or
      - a *derived* edge = `isometry(source edge)`, optionally reversed.
      - Self-related edges (**C**, and later mirror-bisected **t**) are a source half
        plus its derived image.
- [x] **Isometry type**: replace rotation+translation with a general 2D affine
      isometry (rotation, translation, reflection / glide reflection),
      including correct inverse for `cpoint`, `hitpoint`, `hitline`.
- [x] **Symmetry/layout description** on the figure, replacing
      `gridincx/gridincy/shiftx/shifty/rotdiv/is_reversed`:
      - two lattice vectors `a`, `b`
      - a list of *aspect* transforms (one per tile orientation in the unit cell)
      - an orientation → colour index map (min colours from tiled.art)
- [x] `TessellationFigure::placements` returns placements (`lattice point × aspect transform`)
      instead of points; covering logic keeps the radius padding.
- [x] Renderers (svg, tiny-skia, lyon/wgpu) consume full transforms
      (`OutputPrimitive` gets a 2×3 matrix instead of `angle`; update
      `tir-wgpu/shaders/geometry.vs.wgsl` accordingly) and use the figure's colour map
      instead of the brick/hexagon/diamond heuristics.
- [x] Serde: new `lines`/`links`/`colors`/`ih` JSON format; removed the legacy
      line fields and the unused `TessellationShape`. Old JSON is not supported
      and there is no version field; see the "Figure JSON" section of the README.
- [x] Port the original `square`, `hexagon`, `diamond`, `square90` and
      `triangle` figures to the new model.
- [x] Generalise `tests/test_grid.rs::test_grid_covers_area_exactly_once` to use the
      aspect transforms (incl. reflections) and run it for every figure.
- [x] Add a `TessellationFigure::all()` / registry (id, name, Heesch, IH, group)
      so UIs and tests iterate over one list.
- [x] Small fixes: doc comment of `diamond()` says "hexagon";
      rename figures to tiled.art ids (`quad1`, `hex1`, `quad3`, `quad4`, `tri6_alone`)
      while keeping the old constructors as aliases.

**Done when:** the 6 existing figures render identically, all tests pass, and
adding a figure needs no change outside its own file plus the registry.

---

## Milestone 1 – 2-way rotation (p2), no flips

Needs: **C** edges (180° rotation about edge midpoint), 2 aspects.

| | Symmetry | tiled.art id | Heesch | IH | Colours |
|---|---|---|---|---|---|
| ✅ | Triangles, rotated | Tri2 | CCC | 84 | 2 |
| ✅ | Parallelograms, rotated | Quad2Edge | TCTC | 47 | 2 |
| ✅ | Quadrilaterals, rotated | Quad2Tip | CCCC | 46 | 2 |
| ✅ | Pentagons, rotated | Pent2 | TCTCC | 23 | 3 |
| ✅ | Hexagons, rotated | Hex2 | TCCTCC | 4 | 3 |

**Done when:** all five pass the coverage and edge-match tests and are selectable in the UIs.

---

## Milestone 2 – 3-, 4- and 6-way rotation (p3, p4, p6), remaining types

Needs: **C3/C4/C6** edges about *different* vertices in one tile (multiple rotation
centres), mixed with **C** edges.

| | Symmetry | tiled.art id | Heesch | IH | Colours | Rigid |
|---|---|---|---|---|---|---|
| ✅ | Hexagons with 3-way rotation | Hex3 | C3C3C3C3C3C3 | 7 | 3 | |
| ✅ | Triangles with 4-way rotation | Tri4 | CC4C4 | 79 | 2 | • |
| ✅ | Pentagons with 4-way rotation (Cairo) | Pent4 | CC4C4C4C4 | 28 | 3 | |
| ✅ | Isosceles triangles with 6-way rotation | Tri6Tip | CC3C3 | 39 | 3 | • |
| ✅ | Kites with 6-way rotation | Quad6 | C3C3C6C6 | 31 | 3 | • |
| ✅ | Pentagons with 6-way rotation | Pent6 | CC3C3C6C6 | 21 | 3 | |

**Done when:** all six pass the tests. After this milestone all 16 rotation-only
(p1, p2, p3, p4, p6) Heesch types are done.

---

## Milestone 3 – Glide reflection (pg)

Needs: **G** edges (glide reflection), flipped aspects, renderer support for mirrored
copies, editor `corrp` hit-testing through a reflection.

| | Symmetry | tiled.art id | Heesch | IH | Colours |
|---|---|---|---|---|---|
| ✅ | Parallelograms, flipped | Quad2FlipEdge | TGTG | 43 | 2 |
| ✅ | Kites, flipped | Quad2FlipTip | G1G1G2G2 | 44 | 2 |
| ✅ | Hexagons, flipped (on perpendicular axis) | Hex2FlipEnds | TG1G2TG2G1 | 3 | 3 |
| ✅ | Hexagons, flipped (on parallel axis) | Hex2FlipSides | TG1G1TG2G2 | 2 | 3 |

**Done when:** all four pass the tests, and flipped tiles render correctly in SVG,
tiny-skia and wgpu output.

---

## Milestone 4 – Rotation + glide reflection (pgg)

Needs: **C** and **G** edges combined, 4 aspects (identity, 180° rotation, two flips).

| | Symmetry | tiled.art id | Heesch | IH | Colours |
|---|---|---|---|---|---|
| ✅ | Triangles, rotated and flipped | Tri4Flip | CGG | 86 | 2 |
| ✅ | Quadrilaterals (touching), rotated and flipped | Quad4FlipTip | CCGG | 53 | 2 |
| ✅ | Quadrilaterals (isolated), rotated and flipped | Quad4FlipAloneRotate | CGCG | 51 | 2 |
| ✅ | Rectangles with 2-way flip | Quad4FlipAloneFlip | G1G2G1G2 | 52 | 2 |
| ✅ | Pentagons, rotated and flipped | Pent4FlipEdge | TCTGG | 25 | 3 |
| ✅ | Pentagons with 2-way flip | Pent4FlipAlone | CG1G2G1G2 | 27 | 3 |
| ✅ | Hexagons, rotated and flipped | Hex4FlipEdge | TCCTGG | 5 | 3 |
| ✅ | Hexagons with 2-way flip | Hex4FlipAlone | CG1CG2G1G2 | 6 | 3 |

**Done when:** all eight pass the tests. After this milestone all 28 Heesch types are done.

---

## Milestone 5 – Tiles with an inner mirror (cm, pmg, p31m, p4g)

Needs: an internal mirror axis in the figure.
- **t**: an edge bisected by the mirror; only half of it is editable, the other half
  is its reflection.
- **|**: mirror between two edges; the edge on one side is the reflection of the
  edge on the other side.
- Editor: points on the mirror axis stay on the axis; editing one side updates the
  mirrored side.

| | Symmetry | tiled.art id | Heesch | IH | Group | Colours | Rigid |
|---|---|---|---|---|---|---|---|
| ✅ | Rhombuses, with inner mirror | Quad1Sym | TT\|TT | 68 | cm | 2 | |
| ✅ | Hexagons, with inner mirror | Hex1Sym | tTTtTT | 12 | cm | 3 | |
| ✅ | Rectangles (with inner mirror), rotated | Quad2EdgeSym | tCtC | 66 | pmg | 2 | |
| ✅ | Kites (with inner mirror), rotated | Quad2TipSym | CC\|CC | 69 | pmg | 2 | |
| ✅ | Hexagons (with inner mirror), rotated | Hex2Sym | tCCtCC | 13 | pmg | 3 | |
| ✅ | Rhombuses, with inner mirror and 3-way rotation | Quad3Sym | C3C3\|C3C3 | 36 | p31m | 3 | • |
| ✅ | Squares, with inner mirror and 4-way rotation | Quad4Sym | C4C4\|C4C4 | 71 | p4g | 2 | • |

**Done when:** all seven pass the tests and the editor keeps tiles symmetric.
After this milestone all 35 symmetries are done.

---

## Milestone 6 – UI, render polish, editing of base polygons, docs

- [x] Figure cycling in `tir`, `tir-web` and `tir-wgpu` driven by the registry:
      `n` / `p` step forwards/backwards through all 35. All three start on the
      square.
- [x] Edge editing keeps paired and mirrored edges in sync; dragging an
      existing point moves its copies instead of adding new points.
- [x] Show the Heesch code, IH number and wallpaper group in the window title
      (and print the same line from `tir-wgpu`).
- [x] `tir-wgpu` fixes for the full registry: dense tilings fit the primitive
      buffer, reflected tiles are not culled, and 2-/3-colour tilings get
      distinct Alhambra palette colours.
- [x] Add a subtle grain to the `tir-wgpu` fragment shader.
- [x] Share palettes between the renderers (`render/palette.rs`): Alhambra,
      Classic and Grayscale. `tir-web` has a **Colours** menu and `c` cycles
      them; `tir-wgpu` starts on Alhambra and `r` rotates it.
- [x] Tile picker in `tir-web`: a **Tile** menu listed all 35 figures grouped by
      shape (`figure_groups()`), ordered by lowest IH and then by variant IH.
      Superseded by the gallery page below, so the editor no longer has a tile
      menu; `tir`, `tir-web` and `tir-wgpu` cycle with `n`/`p`.
- [x] Gallery page in `tir-web`: the app opens on all 35 symmetries grouped by
      the tiled.art chart categories (`figure_categories()`), each with a
      thumbnail of the tiling; clicking a card starts the editor on that
      figure. The gallery and editor are the `#/` and `#/edit/<id>` routes of
      the same page, so the browser back and forward buttons work.
- [x] Centre the editable tile in `tir-web`: the editor and the gallery
      thumbnails shift the view onto the figure's bounding box, so the tile you
      drag sits in the middle of the drawing. The view is frozen when the figure
      loads, so editing doesn't move it under the pointer.
- [x] Tile picker in `tir-app`: the hamburger menu's figure list (Square,
      Triangle, Square 90°, Diamond, Hexagon) is replaced by a **Pick
      symmetry…** entry that opens a window with all 35 figures grouped by the
      tiled.art chart categories (`figure_categories()`), each shown as a
      thumbnail. Thumbnails use the same fixed zoom as `tir-web` (a four-unit
      window centred on the figure's bounding box) and outline the editable tile
      the same way (a white halo under a dark line). They are triangulated once
      per palette and cached.
- [ ] Non-rigid types: allow moving base-polygon vertices (within the
      constraints of the type) instead of only edge points.
- [x] Rebuild `docs/` (web demo) with all figures; the new bundle includes the
      gallery page and the **Colours** menu.
- [x] README: full supported-symmetry table (id, tile name, Heesch, IH, group
      and colour count for all 35 figures), alongside the existing JSON format,
      title, `tir-web` menu and `tir-wgpu` palette notes.
- [ ] Publish a new `tessellations` crate version (breaking change from Milestone 0).

---

## Overview

| Milestone | New symmetries | Running total |
|---|---|---|
| Existing | Quad1, Hex1, Quad3, Quad4, Tri6Alone | 5 / 35 |
| M0 – core model | Tri2 added with the registry; Hex1/Tri6Alone ids renamed | 6 / 35 |
| M1 – p2 | Quad2Edge, Quad2Tip, Pent2, Hex2 | 10 / 35 |
| M2 – p3/p4/p6 | Hex3, Tri4, Pent4, Tri6Tip, Quad6, Pent6 | 16 / 35 |
| M3 – pg | Quad2FlipEdge, Quad2FlipTip, Hex2FlipEnds, Hex2FlipSides | 20 / 35 |
| M4 – pgg | Tri4Flip, Quad4FlipTip, Quad4FlipAloneRotate, Quad4FlipAloneFlip, Pent4FlipEdge, Pent4FlipAlone, Hex4FlipEdge, Hex4FlipAlone | 28 / 35 |
| M5 – mirror | Quad1Sym, Hex1Sym, Quad2EdgeSym, Quad2TipSym, Hex2Sym, Quad3Sym, Quad4Sym | 35 / 35 |
| M6 – UI, render polish & docs | – | 35 / 35 |

## References

- https://tiled.art/en/symmetryClassification/
- https://tiled.art/en/symmetrySummary/
- Craig Kaplan, Tactile (all 93 isohedral types): https://isohedral.ca/software/tactile/ ,
  https://github.com/isohedral/tactile-js , https://crates.io/crates/tactile
