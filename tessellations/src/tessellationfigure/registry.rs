use crate::tessellationfigure::TessellationFigure;

/// Display metadata for one figure in [`TessellationFigure::all`] order.
pub struct FigureInfo {
    /// Id used by [`TessellationFigure::id_index`], e.g. `"quad2flipedge"`.
    pub id: &'static str,
    /// Human readable symmetry name, as used on tiled.art.
    pub name: &'static str,
    /// Heesch type of the symmetry, in ASCII, e.g. `"CC4C4C4C4"`.
    pub heesch: &'static str,
    /// Isohedral tiling type number (1-93).
    pub ih: u8,
    /// Wallpaper group, e.g. `"p4"`.
    pub group: &'static str,
    /// Number of colours ("Ways" on tiled.art).
    pub colors: usize,
}

/// All 35 tiled.art tessellation symmetries, in [`TessellationFigure::all`] order.
pub const FIGURES: &[FigureInfo] = &[
    // p1
    FigureInfo {
        id: "quad1",
        name: "Parallelograms",
        heesch: "TTTT",
        ih: 41,
        group: "p1",
        colors: 2,
    },
    FigureInfo {
        id: "hex1",
        name: "Hexagons",
        heesch: "TTTTTT",
        ih: 1,
        group: "p1",
        colors: 3,
    },
    // p2
    FigureInfo {
        id: "tri2",
        name: "Triangles, rotated",
        heesch: "CCC",
        ih: 84,
        group: "p2",
        colors: 2,
    },
    FigureInfo {
        id: "quad2edge",
        name: "Parallelograms, rotated",
        heesch: "TCTC",
        ih: 47,
        group: "p2",
        colors: 2,
    },
    FigureInfo {
        id: "quad2tip",
        name: "Quadrilaterals, rotated",
        heesch: "CCCC",
        ih: 46,
        group: "p2",
        colors: 2,
    },
    FigureInfo {
        id: "pent2",
        name: "Pentagons, rotated",
        heesch: "TCTCC",
        ih: 23,
        group: "p2",
        colors: 3,
    },
    FigureInfo {
        id: "hex2",
        name: "Hexagons, rotated",
        heesch: "TCCTCC",
        ih: 4,
        group: "p2",
        colors: 3,
    },
    // p3
    FigureInfo {
        id: "quad3",
        name: "Rhombuses with 3-way rotation",
        heesch: "C3C3C3C3",
        ih: 33,
        group: "p3",
        colors: 3,
    },
    FigureInfo {
        id: "hex3",
        name: "Hexagons with 3-way rotation",
        heesch: "C3C3C3C3C3C3",
        ih: 7,
        group: "p3",
        colors: 3,
    },
    // p4
    FigureInfo {
        id: "tri4",
        name: "Triangles with 4-way rotation",
        heesch: "CC4C4",
        ih: 79,
        group: "p4",
        colors: 2,
    },
    FigureInfo {
        id: "quad4",
        name: "Squares with 4-way rotation",
        heesch: "C4C4C4C4",
        ih: 55,
        group: "p4",
        colors: 2,
    },
    FigureInfo {
        id: "pent4",
        name: "Pentagons with 4-way rotation",
        heesch: "CC4C4C4C4",
        ih: 28,
        group: "p4",
        colors: 3,
    },
    // p6
    FigureInfo {
        id: "tri6alone",
        name: "Equilateral triangles with 6-way rotation",
        heesch: "CC6C6",
        ih: 88,
        group: "p6",
        colors: 2,
    },
    FigureInfo {
        id: "tri6tip",
        name: "Isosceles triangles with 6-way rotation",
        heesch: "CC3C3",
        ih: 39,
        group: "p6",
        colors: 3,
    },
    FigureInfo {
        id: "quad6",
        name: "Kites with 6-way rotation",
        heesch: "C3C3C6C6",
        ih: 31,
        group: "p6",
        colors: 3,
    },
    FigureInfo {
        id: "pent6",
        name: "Pentagons with 6-way rotation",
        heesch: "CC3C3C6C6",
        ih: 21,
        group: "p6",
        colors: 3,
    },
    // pg
    FigureInfo {
        id: "quad2flipedge",
        name: "Parallelograms, flipped",
        heesch: "TGTG",
        ih: 43,
        group: "pg",
        colors: 2,
    },
    FigureInfo {
        id: "quad2fliptip",
        name: "Kites, flipped",
        heesch: "G1G1G2G2",
        ih: 44,
        group: "pg",
        colors: 2,
    },
    FigureInfo {
        id: "hex2flipend",
        name: "Hexagons, flipped (on perpendicular axis)",
        heesch: "TG1G2TG2G1",
        ih: 3,
        group: "pg",
        colors: 3,
    },
    FigureInfo {
        id: "hex2flipsides",
        name: "Hexagons, flipped (on parallel axis)",
        heesch: "TG1G1TG2G2",
        ih: 2,
        group: "pg",
        colors: 3,
    },
    // pgg
    FigureInfo {
        id: "tri4flip",
        name: "Triangles, rotated and flipped",
        heesch: "CGG",
        ih: 86,
        group: "pgg",
        colors: 2,
    },
    FigureInfo {
        id: "quad4fliptip",
        name: "Quadrilaterals (touching), rotated and flipped",
        heesch: "CCGG",
        ih: 53,
        group: "pgg",
        colors: 2,
    },
    FigureInfo {
        id: "quad4flipalonerotate",
        name: "Quadrilaterals (isolated), rotated and flipped",
        heesch: "CGCG",
        ih: 51,
        group: "pgg",
        colors: 2,
    },
    FigureInfo {
        id: "quad4flipaloneflip",
        name: "Rectangles with 2-way flip",
        heesch: "G1G2G1G2",
        ih: 52,
        group: "pgg",
        colors: 2,
    },
    FigureInfo {
        id: "pent4flipedge",
        name: "Pentagons, rotated and flipped",
        heesch: "TCTGG",
        ih: 25,
        group: "pgg",
        colors: 3,
    },
    FigureInfo {
        id: "pent4flipalone",
        name: "Pentagons with 2-way flip",
        heesch: "CG1G2G1G2",
        ih: 27,
        group: "pgg",
        colors: 3,
    },
    FigureInfo {
        id: "hex4flipedge",
        name: "Hexagons, rotated and flipped",
        heesch: "TCCTGG",
        ih: 5,
        group: "pgg",
        colors: 3,
    },
    FigureInfo {
        id: "hex4flipalone",
        name: "Hexagons with 2-way flip",
        heesch: "CG1CG2G1G2",
        ih: 6,
        group: "pgg",
        colors: 3,
    },
    // inner mirror (cm, pmg, p31m, p4g)
    FigureInfo {
        id: "quad1sym",
        name: "Rhombuses, with inner mirror",
        heesch: "TT|TT",
        ih: 68,
        group: "cm",
        colors: 2,
    },
    FigureInfo {
        id: "hex1sym",
        name: "Hexagons, with inner mirror",
        heesch: "tTTtTT",
        ih: 12,
        group: "cm",
        colors: 3,
    },
    FigureInfo {
        id: "quad2edgesym",
        name: "Rectangles (with inner mirror), rotated",
        heesch: "tCtC",
        ih: 66,
        group: "pmg",
        colors: 2,
    },
    FigureInfo {
        id: "quad2tipsym",
        name: "Kites (with inner mirror), rotated",
        heesch: "CC|CC",
        ih: 69,
        group: "pmg",
        colors: 2,
    },
    FigureInfo {
        id: "hex2sym",
        name: "Hexagons (with inner mirror), rotated",
        heesch: "tCCtCC",
        ih: 13,
        group: "pmg",
        colors: 3,
    },
    FigureInfo {
        id: "quad3sym",
        name: "Rhombuses, with inner mirror and 3-way rotation",
        heesch: "C3C3|C3C3",
        ih: 36,
        group: "p31m",
        colors: 3,
    },
    FigureInfo {
        id: "quad4sym",
        name: "Squares, with inner mirror and 4-way rotation",
        heesch: "C4C4|C4C4",
        ih: 71,
        group: "p4g",
        colors: 2,
    },
];

/// Ids of the figures returned by [`TessellationFigure::all`], in the same order.
pub const FIGURE_IDS: &[&str] = &[
    "quad1",
    "hex1",
    "tri2",
    "quad2edge",
    "quad2tip",
    "pent2",
    "hex2",
    "quad3",
    "hex3",
    "tri4",
    "quad4",
    "pent4",
    "tri6alone",
    "tri6tip",
    "quad6",
    "pent6",
    "quad2flipedge",
    "quad2fliptip",
    "hex2flipend",
    "hex2flipsides",
    "tri4flip",
    "quad4fliptip",
    "quad4flipalonerotate",
    "quad4flipaloneflip",
    "pent4flipedge",
    "pent4flipalone",
    "hex4flipedge",
    "hex4flipalone",
    "quad1sym",
    "hex1sym",
    "quad2edgesym",
    "quad2tipsym",
    "hex2sym",
    "quad3sym",
    "quad4sym",
];

impl TessellationFigure {
    /// All 35 tessellation figures, in [`FIGURE_IDS`] order.
    pub fn all() -> Vec<Self> {
        vec![
            Self::square(),
            Self::hexagon(),
            Self::triangle(),
            Self::quad2edge(),
            Self::quad2tip(),
            Self::pent2(),
            Self::hex2(),
            Self::diamond(),
            Self::hex3(),
            Self::tri4(),
            Self::square90(),
            Self::pent4(),
            Self::tri6alone(),
            Self::tri6tip(),
            Self::quad6(),
            Self::pent6(),
            Self::quad2flipedge(),
            Self::quad2fliptip(),
            Self::hex2flipend(),
            Self::hex2flipsides(),
            Self::tri4flip(),
            Self::quad4fliptip(),
            Self::quad4flipalonerotate(),
            Self::quad4flipaloneflip(),
            Self::pent4flipedge(),
            Self::pent4flipalone(),
            Self::hex4flipedge(),
            Self::hex4flipalone(),
            Self::quad1sym(),
            Self::hex1sym(),
            Self::quad2edgesym(),
            Self::quad2tipsym(),
            Self::hex2sym(),
            Self::quad3sym(),
            Self::quad4sym(),
        ]
    }

    /// Index of this figure in [`TessellationFigure::all`], matched by id.
    pub fn id_index(id: &str) -> Option<usize> {
        FIGURE_IDS.iter().position(|&i| i == id)
    }

    /// Figure at `index` in [`TessellationFigure::all`] order; wraps around.
    pub fn by_index(index: usize) -> Self {
        let index = index % FIGURE_IDS.len();
        Self::all().swap_remove(index)
    }

    /// Window title for the figure at `index`, e.g.
    /// `"Tessellations of Hexagons (TTTTTT)"`.
    pub fn title(index: usize) -> String {
        let info = &FIGURES[index % FIGURES.len()];
        format!("Tessellations of {} ({})", info.name, info.heesch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_matches_ids() {
        assert_eq!(TessellationFigure::all().len(), FIGURE_IDS.len());
        assert_eq!(FIGURES.len(), FIGURE_IDS.len());
        assert_eq!(FIGURE_IDS.len(), 35);
    }

    #[test]
    fn test_ids_match_metadata() {
        for (i, info) in FIGURES.iter().enumerate() {
            assert_eq!(info.id, FIGURE_IDS[i]);
        }
    }

    #[test]
    fn test_square_is_first() {
        assert_eq!(FIGURE_IDS[0], "quad1");
        assert_eq!(
            TessellationFigure::title(0),
            "Tessellations of Parallelograms (TTTT)"
        );
    }

    #[test]
    fn test_title_wraps() {
        assert_eq!(
            TessellationFigure::title(FIGURE_IDS.len()),
            TessellationFigure::title(0)
        );
        assert_eq!(
            TessellationFigure::title(1),
            "Tessellations of Hexagons (TTTTTT)"
        );
    }

    #[test]
    fn test_by_index_wraps() {
        let first = TessellationFigure::by_index(0);
        let wrapped = TessellationFigure::by_index(FIGURE_IDS.len());
        assert_eq!(
            serde_json::to_string(&first).unwrap(),
            serde_json::to_string(&wrapped).unwrap()
        );
    }

    #[test]
    fn test_ids_are_unique() {
        for (i, id) in FIGURE_IDS.iter().enumerate() {
            assert_eq!(TessellationFigure::id_index(id), Some(i));
        }
    }
}
