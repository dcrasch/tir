//! Per-type colouring data from [tactile-rs](https://github.com/abusch/tactile-rs).
//!
//! Each isohedral tiling type has a canonical colouring of its tiles: a base
//! colour per aspect (tile orientation) plus a colour transition for stepping
//! one unit along each of the two lattice vectors. The layout matches the
//! `colouring` arrays in tactile-rs: twelve aspect base colours, two
//! three-entry transition tables, then the number of colours.
use serde::{Deserialize, Serialize};

/// Colouring table of one isohedral tiling type.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ColorTable {
    /// Isohedral tiling type number (1-93).
    pub ih: u8,
    /// Number of aspects (tile poses per lattice cell).
    pub aspects: usize,
    /// 12 aspect colours, two 3-entry transitions, colour count.
    pub table: [u8; 19],
}

impl ColorTable {
    /// Number of colours in this colouring.
    pub fn colors(&self) -> usize {
        self.table[18] as usize
    }

    /// Colour of the tile at lattice coordinates `(a, b)` with `aspect`.
    pub fn color(&self, aspect: usize, a: i64, b: i64) -> usize {
        let n = self.colors().max(1) as i64;
        let mut c = self.table[aspect % self.aspects.max(1)] as usize;
        for _ in 0..a.rem_euclid(n) {
            c = self.table[12 + c] as usize;
        }
        for _ in 0..b.rem_euclid(n) {
            c = self.table[15 + c] as usize;
        }
        c
    }
}

/// Colour table for isohedral tiling type `ih`, if known.
pub fn color_table(ih: u8) -> Option<&'static ColorTable> {
    TABLES.iter().find(|t| t.ih == ih)
}

/// Colour tables for the tiled.art symmetry types.
pub static TABLES: &[ColorTable] = &[
    ColorTable {
        ih: 41,
        aspects: 1,
        table: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 1, 0, 2, 2],
    },
    ColorTable {
        ih: 1,
        aspects: 1,
        table: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 0, 1, 3],
    },
    ColorTable {
        ih: 84,
        aspects: 2,
        table: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2],
    },
    ColorTable {
        ih: 47,
        aspects: 2,
        table: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 0, 1, 2, 2],
    },
    ColorTable {
        ih: 46,
        aspects: 2,
        table: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2],
    },
    ColorTable {
        ih: 23,
        aspects: 2,
        table: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 0, 1, 3],
    },
    ColorTable {
        ih: 4,
        aspects: 2,
        table: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 2, 0, 1, 3],
    },
    ColorTable {
        ih: 33,
        aspects: 3,
        table: [0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 3],
    },
    ColorTable {
        ih: 7,
        aspects: 3,
        table: [0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 3],
    },
    ColorTable {
        ih: 79,
        aspects: 4,
        table: [0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 1, 0, 2, 2],
    },
    ColorTable {
        ih: 55,
        aspects: 4,
        table: [0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2],
    },
    ColorTable {
        ih: 28,
        aspects: 4,
        table: [0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 1, 2, 3],
    },
    ColorTable {
        ih: 88,
        aspects: 6,
        table: [0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2],
    },
    ColorTable {
        ih: 39,
        aspects: 6,
        table: [0, 2, 1, 0, 1, 2, 0, 0, 0, 0, 0, 0, 2, 0, 1, 1, 2, 0, 3],
    },
    ColorTable {
        ih: 31,
        aspects: 6,
        table: [0, 1, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 3],
    },
    ColorTable {
        ih: 21,
        aspects: 6,
        table: [0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 0, 1, 3],
    },
    ColorTable {
        ih: 43,
        aspects: 2,
        table: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 0, 1, 2, 2],
    },
    ColorTable {
        ih: 44,
        aspects: 2,
        table: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2],
    },
    ColorTable {
        ih: 3,
        aspects: 2,
        table: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 2, 0, 1, 3],
    },
    ColorTable {
        ih: 2,
        aspects: 2,
        table: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 1, 2, 3],
    },
    ColorTable {
        ih: 86,
        aspects: 4,
        table: [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2],
    },
    ColorTable {
        ih: 53,
        aspects: 4,
        table: [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2],
    },
    ColorTable {
        ih: 51,
        aspects: 4,
        table: [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2],
    },
    ColorTable {
        ih: 52,
        aspects: 4,
        table: [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2],
    },
    ColorTable {
        ih: 25,
        aspects: 4,
        table: [0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 0, 1, 2, 3],
    },
    ColorTable {
        ih: 27,
        aspects: 4,
        table: [0, 1, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 1, 2, 0, 3],
    },
    ColorTable {
        ih: 5,
        aspects: 4,
        table: [0, 1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 2, 0, 1, 3],
    },
    ColorTable {
        ih: 6,
        aspects: 4,
        table: [0, 1, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 1, 2, 3],
    },
    ColorTable {
        ih: 68,
        aspects: 1,
        table: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 1, 0, 2, 2],
    },
    ColorTable {
        ih: 12,
        aspects: 1,
        table: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 0, 1, 3],
    },
    ColorTable {
        ih: 66,
        aspects: 2,
        table: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 0, 1, 2, 2],
    },
    ColorTable {
        ih: 69,
        aspects: 2,
        table: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2],
    },
    ColorTable {
        ih: 13,
        aspects: 2,
        table: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 0, 1, 2, 3],
    },
    ColorTable {
        ih: 36,
        aspects: 3,
        table: [0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 3],
    },
    ColorTable {
        ih: 71,
        aspects: 4,
        table: [0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 1, 2, 2],
    },
];
