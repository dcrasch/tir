use crate::tessellationfigure::TessellationFigure;

/// Display metadata for one figure in [`TessellationFigure::all`] order.
pub struct FigureInfo {
    /// Id used by [`TessellationFigure::id_index`], e.g. `"quad2flipedge"`.
    pub id: &'static str,
    /// Shape of the tile, e.g. `"Hexagons"`. A tile menu groups by this.
    pub shape: &'static str,
    /// How the figure differs from the plain shape, e.g. `"rotated"`. Empty
    /// when the figure is the plain shape.
    pub variant: &'static str,
    /// Heesch type of the symmetry, in ASCII, e.g. `"CC4C4C4C4"`.
    pub heesch: &'static str,
    /// Isohedral tiling type number (1-93).
    pub ih: u8,
    /// Wallpaper group, e.g. `"p4"`.
    pub group: &'static str,
    /// Number of colours ("Ways" on tiled.art).
    pub colors: usize,
}

impl FigureInfo {
    /// Full display name, e.g. `"Triangles, rotated"` or `"Hexagons"`.
    pub fn name(&self) -> String {
        if self.variant.is_empty() {
            self.shape.to_string()
        } else {
            format!("{}, {}", self.shape, self.variant)
        }
    }
}

/// All 35 tiled.art tessellation symmetries, in [`TessellationFigure::all`] order.
pub const FIGURES: &[FigureInfo] = &[
    // p1
    FigureInfo {
        id: "quad1",
        shape: "Parallelograms",
        variant: "",
        heesch: "TTTT",
        ih: 41,
        group: "p1",
        colors: 2,
    },
    FigureInfo {
        id: "hex1",
        shape: "Hexagons",
        variant: "",
        heesch: "TTTTTT",
        ih: 1,
        group: "p1",
        colors: 3,
    },
    // p2
    FigureInfo {
        id: "tri2",
        shape: "Triangles",
        variant: "rotated",
        heesch: "CCC",
        ih: 84,
        group: "p2",
        colors: 2,
    },
    FigureInfo {
        id: "quad2edge",
        shape: "Parallelograms",
        variant: "rotated",
        heesch: "TCTC",
        ih: 47,
        group: "p2",
        colors: 2,
    },
    FigureInfo {
        id: "quad2tip",
        shape: "Quadrilaterals",
        variant: "rotated",
        heesch: "CCCC",
        ih: 46,
        group: "p2",
        colors: 2,
    },
    FigureInfo {
        id: "pent2",
        shape: "Pentagons",
        variant: "rotated",
        heesch: "TCTCC",
        ih: 23,
        group: "p2",
        colors: 3,
    },
    FigureInfo {
        id: "hex2",
        shape: "Hexagons",
        variant: "rotated",
        heesch: "TCCTCC",
        ih: 4,
        group: "p2",
        colors: 3,
    },
    // p3
    FigureInfo {
        id: "quad3",
        shape: "Rhombuses",
        variant: "3-way rotation",
        heesch: "C3C3C3C3",
        ih: 33,
        group: "p3",
        colors: 3,
    },
    FigureInfo {
        id: "hex3",
        shape: "Hexagons",
        variant: "3-way rotation",
        heesch: "C3C3C3C3C3C3",
        ih: 7,
        group: "p3",
        colors: 3,
    },
    // p4
    FigureInfo {
        id: "tri4",
        shape: "Triangles",
        variant: "4-way rotation",
        heesch: "CC4C4",
        ih: 79,
        group: "p4",
        colors: 2,
    },
    FigureInfo {
        id: "quad4",
        shape: "Squares",
        variant: "4-way rotation",
        heesch: "C4C4C4C4",
        ih: 55,
        group: "p4",
        colors: 2,
    },
    FigureInfo {
        id: "pent4",
        shape: "Pentagons",
        variant: "4-way rotation",
        heesch: "CC4C4C4C4",
        ih: 28,
        group: "p4",
        colors: 3,
    },
    // p6
    FigureInfo {
        id: "tri6alone",
        shape: "Equilateral triangles",
        variant: "6-way rotation",
        heesch: "CC6C6",
        ih: 88,
        group: "p6",
        colors: 2,
    },
    FigureInfo {
        id: "tri6tip",
        shape: "Isosceles triangles",
        variant: "6-way rotation",
        heesch: "CC3C3",
        ih: 39,
        group: "p6",
        colors: 3,
    },
    FigureInfo {
        id: "quad6",
        shape: "Kites",
        variant: "6-way rotation",
        heesch: "C3C3C6C6",
        ih: 31,
        group: "p6",
        colors: 3,
    },
    FigureInfo {
        id: "pent6",
        shape: "Pentagons",
        variant: "6-way rotation",
        heesch: "CC3C3C6C6",
        ih: 21,
        group: "p6",
        colors: 3,
    },
    // pg
    FigureInfo {
        id: "quad2flipedge",
        shape: "Parallelograms",
        variant: "flipped",
        heesch: "TGTG",
        ih: 43,
        group: "pg",
        colors: 2,
    },
    FigureInfo {
        id: "quad2fliptip",
        shape: "Kites",
        variant: "flipped",
        heesch: "G1G1G2G2",
        ih: 44,
        group: "pg",
        colors: 2,
    },
    FigureInfo {
        id: "hex2flipend",
        shape: "Hexagons",
        variant: "flipped (on perpendicular axis)",
        heesch: "TG1G2TG2G1",
        ih: 3,
        group: "pg",
        colors: 3,
    },
    FigureInfo {
        id: "hex2flipsides",
        shape: "Hexagons",
        variant: "flipped (on parallel axis)",
        heesch: "TG1G1TG2G2",
        ih: 2,
        group: "pg",
        colors: 3,
    },
    // pgg
    FigureInfo {
        id: "tri4flip",
        shape: "Triangles",
        variant: "rotated and flipped",
        heesch: "CGG",
        ih: 86,
        group: "pgg",
        colors: 2,
    },
    FigureInfo {
        id: "quad4fliptip",
        shape: "Quadrilaterals",
        variant: "touching, rotated and flipped",
        heesch: "CCGG",
        ih: 53,
        group: "pgg",
        colors: 2,
    },
    FigureInfo {
        id: "quad4flipalonerotate",
        shape: "Quadrilaterals",
        variant: "isolated, rotated and flipped",
        heesch: "CGCG",
        ih: 51,
        group: "pgg",
        colors: 2,
    },
    FigureInfo {
        id: "quad4flipaloneflip",
        shape: "Rectangles",
        variant: "2-way flip",
        heesch: "G1G2G1G2",
        ih: 52,
        group: "pgg",
        colors: 2,
    },
    FigureInfo {
        id: "pent4flipedge",
        shape: "Pentagons",
        variant: "rotated and flipped",
        heesch: "TCTGG",
        ih: 25,
        group: "pgg",
        colors: 3,
    },
    FigureInfo {
        id: "pent4flipalone",
        shape: "Pentagons",
        variant: "2-way flip",
        heesch: "CG1G2G1G2",
        ih: 27,
        group: "pgg",
        colors: 3,
    },
    FigureInfo {
        id: "hex4flipedge",
        shape: "Hexagons",
        variant: "rotated and flipped",
        heesch: "TCCTGG",
        ih: 5,
        group: "pgg",
        colors: 3,
    },
    FigureInfo {
        id: "hex4flipalone",
        shape: "Hexagons",
        variant: "2-way flip",
        heesch: "CG1CG2G1G2",
        ih: 6,
        group: "pgg",
        colors: 3,
    },
    // inner mirror (cm, pmg, p31m, p4g)
    FigureInfo {
        id: "quad1sym",
        shape: "Rhombuses",
        variant: "with inner mirror",
        heesch: "TT|TT",
        ih: 68,
        group: "cm",
        colors: 2,
    },
    FigureInfo {
        id: "hex1sym",
        shape: "Hexagons",
        variant: "with inner mirror",
        heesch: "tTTtTT",
        ih: 12,
        group: "cm",
        colors: 3,
    },
    FigureInfo {
        id: "quad2edgesym",
        shape: "Rectangles",
        variant: "with inner mirror, rotated",
        heesch: "tCtC",
        ih: 66,
        group: "pmg",
        colors: 2,
    },
    FigureInfo {
        id: "quad2tipsym",
        shape: "Kites",
        variant: "with inner mirror, rotated",
        heesch: "CC|CC",
        ih: 69,
        group: "pmg",
        colors: 2,
    },
    FigureInfo {
        id: "hex2sym",
        shape: "Hexagons",
        variant: "with inner mirror, rotated",
        heesch: "tCCtCC",
        ih: 13,
        group: "pmg",
        colors: 3,
    },
    FigureInfo {
        id: "quad3sym",
        shape: "Rhombuses",
        variant: "with inner mirror and 3-way rotation",
        heesch: "C3C3|C3C3",
        ih: 36,
        group: "p31m",
        colors: 3,
    },
    FigureInfo {
        id: "quad4sym",
        shape: "Squares",
        variant: "with inner mirror and 4-way rotation",
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

/// Figure indices grouped by [`FigureInfo::shape`], for a grouped tile picker.
///
/// Shapes are ordered by their lowest isohedral type, and the figures within a
/// shape by isohedral type, so the list reads roughly by IH number.
pub fn figure_groups() -> Vec<(&'static str, Vec<usize>)> {
    let mut groups: Vec<(&'static str, Vec<usize>)> = Vec::new();
    for (index, info) in FIGURES.iter().enumerate() {
        match groups.iter_mut().find(|(shape, _)| *shape == info.shape) {
            Some((_, members)) => members.push(index),
            None => groups.push((info.shape, vec![index])),
        }
    }
    let ih = |index: usize| FIGURES[index].ih;
    for (_, members) in groups.iter_mut() {
        members.sort_by_key(|&index| ih(index));
    }
    groups.sort_by_key(|(_, members)| members.iter().map(|&index| ih(index)).min());
    groups
}

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
    /// `"Tessellations of Hexagons (TTTTTT, IH 1, p1)"`.
    pub fn title(index: usize) -> String {
        let info = &FIGURES[index % FIGURES.len()];
        format!(
            "Tessellations of {} ({}, IH {}, {})",
            info.name(),
            info.heesch,
            info.ih,
            info.group
        )
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
            "Tessellations of Parallelograms (TTTT, IH 41, p1)"
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
            "Tessellations of Hexagons (TTTTTT, IH 1, p1)"
        );
    }

    #[test]
    fn test_name_joins_shape_and_variant() {
        let name = |id| FIGURES[TessellationFigure::id_index(id).unwrap()].name();
        assert_eq!(name("hex1"), "Hexagons");
        assert_eq!(name("tri2"), "Triangles, rotated");
        assert_eq!(name("quad3"), "Rhombuses, 3-way rotation");
        assert_eq!(
            name("quad4fliptip"),
            "Quadrilaterals, touching, rotated and flipped"
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

    #[test]
    fn test_groups_hold_every_figure_once() {
        let mut seen: Vec<usize> = figure_groups()
            .iter()
            .flat_map(|(_, members)| members.iter().copied())
            .collect();
        seen.sort_unstable();
        assert_eq!(seen, (0..FIGURE_IDS.len()).collect::<Vec<_>>());
    }

    #[test]
    fn test_groups_are_sorted_by_ih() {
        let groups = figure_groups();
        let ih = |index: usize| FIGURES[index].ih;

        let first_ihs: Vec<u8> = groups
            .iter()
            .map(|(_, members)| members.iter().map(|&i| ih(i)).min().unwrap())
            .collect();
        let mut sorted = first_ihs.clone();
        sorted.sort_unstable();
        assert_eq!(first_ihs, sorted, "shapes are not ordered by IH");

        for (shape, members) in &groups {
            let ihs: Vec<u8> = members.iter().map(|&i| ih(i)).collect();
            let mut sorted = ihs.clone();
            sorted.sort_unstable();
            assert_eq!(ihs, sorted, "{shape} is not ordered by IH");
        }
    }

    #[test]
    fn test_groups_keep_a_shape_together() {
        let groups = figure_groups();
        let mut shapes: Vec<&str> = groups.iter().map(|(shape, _)| *shape).collect();
        shapes.sort_unstable();
        let mut deduped = shapes.clone();
        deduped.dedup();
        assert_eq!(shapes, deduped, "a shape appears in more than one group");
    }

    #[test]
    fn test_triangles_group_has_the_variants() {
        let groups = figure_groups();
        let (_, members) = groups
            .iter()
            .find(|(shape, _)| *shape == "Triangles")
            .expect("no Triangles group");
        let variants: Vec<&str> = members.iter().map(|&i| FIGURES[i].variant).collect();
        assert_eq!(
            variants,
            vec!["4-way rotation", "rotated", "rotated and flipped"]
        );
    }
}
