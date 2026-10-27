use crate::tessellationfigure::spec::{from_polygon, from_polygon_with_mirrors, EdgeKind};
use crate::tessellationfigure::TessellationFigure;

const H: f32 = 0.866_025_4;

/// Regular hexagon, vertices counter clockwise from (1, 0).
fn hexagon() -> Vec<(f32, f32)> {
    vec![
        (1.0, 0.0),
        (0.5, H),
        (-0.5, H),
        (-1.0, 0.0),
        (-0.5, -H),
        (0.5, -H),
    ]
}

fn unit_square() -> Vec<(f32, f32)> {
    vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
}

/// Equilateral triangle.
fn triangle() -> Vec<(f32, f32)> {
    vec![(0.0, 0.0), (1.0, 0.0), (0.5, H)]
}

/// Cairo pentagon: angles 120, 90, 120, 90, 120.
fn cairo() -> Vec<(f32, f32)> {
    vec![
        (0.0, 0.0),
        (0.732_050_8, 0.0),
        (1.232_050_8, H),
        (0.366_025_4, 1.366_025_4),
        (-0.5, H),
    ]
}

impl TessellationFigure {
    // ---------------------------------------------------------------- p2
    /// Parallelograms, rotated (TCTC).
    pub fn quad2edge() -> Self {
        from_polygon(
            &[(0.0, 0.0), (1.2, 0.0), (1.5, 0.8), (0.3, 0.8)],
            &[
                (0, 2, EdgeKind::Translate),
                (1, 1, EdgeKind::HalfTurn),
                (3, 3, EdgeKind::HalfTurn),
            ],
            2,
        )
        .with_ih(47)
    }

    /// Quadrilaterals, rotated (CCCC).
    pub fn quad2tip() -> Self {
        from_polygon(
            &[(0.0, 0.0), (1.2, 0.0), (1.4, 0.9), (0.1, 1.1)],
            &[
                (0, 0, EdgeKind::HalfTurn),
                (1, 1, EdgeKind::HalfTurn),
                (2, 2, EdgeKind::HalfTurn),
                (3, 3, EdgeKind::HalfTurn),
            ],
            2,
        )
        .with_ih(46)
    }

    /// Pentagons, rotated (TCTCC).
    pub fn pent2() -> Self {
        from_polygon(
            &[(0.0, 0.0), (1.0, 0.0), (1.5, 1.0), (0.5, 1.0), (0.7, 0.4)],
            &[
                (0, 2, EdgeKind::Translate),
                (1, 1, EdgeKind::HalfTurn),
                (3, 3, EdgeKind::HalfTurn),
                (4, 4, EdgeKind::HalfTurn),
            ],
            3,
        )
        .with_ih(23)
    }

    /// Hexagons, rotated (TCCTCC).
    pub fn hex2() -> Self {
        from_polygon(
            &hexagon(),
            &[
                (0, 3, EdgeKind::Translate),
                (1, 1, EdgeKind::HalfTurn),
                (2, 2, EdgeKind::HalfTurn),
                (4, 4, EdgeKind::HalfTurn),
                (5, 5, EdgeKind::HalfTurn),
            ],
            3,
        )
        .with_ih(4)
    }

    // ---------------------------------------------------------------- p3
    /// Hexagons with 3-way rotation (C3C3C3C3C3C3).
    pub fn hex3() -> Self {
        from_polygon(
            &hexagon(),
            &[
                (0, 1, EdgeKind::Rotate(3)),
                (2, 3, EdgeKind::Rotate(3)),
                (4, 5, EdgeKind::Rotate(3)),
            ],
            3,
        )
        .with_ih(7)
    }

    // ---------------------------------------------------------------- p4
    /// Triangles with 4-way rotation (CC4C4).
    pub fn tri4() -> Self {
        from_polygon(
            &[(1.0, 0.0), (0.0, 1.0), (0.0, 0.0)],
            &[(0, 0, EdgeKind::HalfTurn), (1, 2, EdgeKind::Rotate(4))],
            2,
        )
        .with_ih(79)
    }

    /// Pentagons with 4-way rotation (CC4C4C4C4), Cairo tiling.
    pub fn pent4() -> Self {
        // Base polygon and edge pairing from ih28 (tactile-rs).
        from_polygon(
            &[
                (0.0, 1.0),
                (0.0, 0.0),
                (1.183_013, -0.683_013),
                (1.866_025, 0.5),
                (1.183_013, 1.683_013),
            ],
            &[
                (0, 0, EdgeKind::HalfTurn),
                (1, 2, EdgeKind::Rotate(4)),
                (3, 4, EdgeKind::Rotate(4)),
            ],
            3,
        )
        .with_ih(28)
    }

    // ---------------------------------------------------------------- pg
    /// Parallelograms, flipped (TGTG).
    pub fn quad2flipedge() -> Self {
        from_polygon(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 0.7), (0.0, 0.7)],
            &[(0, 2, EdgeKind::Translate), (1, 3, EdgeKind::Glide)],
            2,
        )
        .with_ih(43)
    }

    /// Kites, flipped (G1G1G2G2).
    pub fn quad2fliptip() -> Self {
        // Base polygon and edge pairing from ih44 (tactile-rs).
        from_polygon(
            &[(0.5, -0.5), (1.0, 0.0), (0.5, 0.5), (0.0, 0.0)],
            &[(0, 1, EdgeKind::Glide), (2, 3, EdgeKind::Glide)],
            2,
        )
        .with_ih(44)
    }

    /// Hexagons, flipped on the perpendicular axis (TG1G2TG2G1).
    pub fn hex2flipend() -> Self {
        from_polygon(
            &hexagon(),
            &[
                (0, 3, EdgeKind::Translate),
                (1, 5, EdgeKind::Glide),
                (2, 4, EdgeKind::Glide),
            ],
            3,
        )
        .with_ih(3)
    }

    /// Hexagons, flipped on the parallel axis (TG1G1TG2G2).
    pub fn hex2flipsides() -> Self {
        from_polygon(
            &hexagon(),
            &[
                (0, 3, EdgeKind::Translate),
                (1, 2, EdgeKind::Glide),
                (4, 5, EdgeKind::Glide),
            ],
            3,
        )
        .with_ih(2)
    }

    // ---------------------------------------------------------------- pgg
    /// Triangles, rotated and flipped (CGG).
    pub fn tri4flip() -> Self {
        from_polygon(
            &triangle(),
            &[(0, 0, EdgeKind::HalfTurn), (1, 2, EdgeKind::Glide)],
            2,
        )
        .with_ih(86)
    }

    /// Quadrilaterals (touching), rotated and flipped (CCGG).
    pub fn quad4fliptip() -> Self {
        from_polygon(
            &unit_square(),
            &[
                (0, 0, EdgeKind::HalfTurn),
                (1, 1, EdgeKind::HalfTurn),
                (2, 3, EdgeKind::Glide),
            ],
            2,
        )
        .with_ih(53)
    }

    /// Quadrilaterals (isolated), rotated and flipped (CGCG).
    pub fn quad4flipalonerotate() -> Self {
        from_polygon(
            &unit_square(),
            &[
                (0, 0, EdgeKind::HalfTurn),
                (1, 3, EdgeKind::Glide),
                (2, 2, EdgeKind::HalfTurn),
            ],
            2,
        )
        .with_ih(51)
    }

    /// Rectangles with 2-way flip (G1G2G1G2).
    pub fn quad4flipaloneflip() -> Self {
        from_polygon(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 0.7), (0.0, 0.7)],
            &[(0, 2, EdgeKind::Glide), (1, 3, EdgeKind::Glide)],
            2,
        )
        .with_ih(52)
    }

    /// Pentagons, rotated and flipped (TCTGG).
    pub fn pent4flipedge() -> Self {
        // Base polygon and edge pairing from ih25 (tactile-rs).
        from_polygon(
            &[
                (1.0, 0.0),
                (1.288_675, 0.5),
                (1.0, 1.0),
                (0.0, 1.0),
                (0.0, 0.0),
            ],
            &[
                (0, 1, EdgeKind::Glide),
                (2, 4, EdgeKind::Translate),
                (3, 3, EdgeKind::HalfTurn),
            ],
            3,
        )
        .with_ih(25)
    }

    /// Pentagons with 2-way flip (CG1G2G1G2), Cairo tiling.
    pub fn pent4flipalone() -> Self {
        from_polygon(
            &cairo(),
            &[
                (0, 0, EdgeKind::HalfTurn),
                (1, 3, EdgeKind::Glide),
                (2, 4, EdgeKind::Glide),
            ],
            3,
        )
        .with_ih(27)
    }

    /// Hexagons, rotated and flipped (TCCTGG).
    pub fn hex4flipedge() -> Self {
        from_polygon(
            &hexagon(),
            &[
                (0, 3, EdgeKind::Translate),
                (1, 1, EdgeKind::HalfTurn),
                (2, 2, EdgeKind::HalfTurn),
                (4, 5, EdgeKind::Glide),
            ],
            3,
        )
        .with_ih(5)
    }

    /// Hexagons with 2-way flip (CG1CG2G1G2).
    pub fn hex4flipalone() -> Self {
        from_polygon(
            &hexagon(),
            &[
                (0, 0, EdgeKind::HalfTurn),
                (1, 4, EdgeKind::Glide),
                (2, 2, EdgeKind::HalfTurn),
                (3, 5, EdgeKind::Glide),
            ],
            3,
        )
        .with_ih(6)
    }

    // ---------------------------------------------------------------- p6
    /// Equilateral triangles with 6-way rotation (CC6C6).
    pub fn tri6alone() -> Self {
        from_polygon(
            &triangle(),
            &[(0, 0, EdgeKind::HalfTurn), (1, 2, EdgeKind::Rotate(6))],
            2,
        )
        .with_ih(88)
    }

    /// Isosceles triangles with 6-way rotation (CC3C3).
    pub fn tri6tip() -> Self {
        from_polygon(
            &[(-0.5, 0.0), (0.5, 0.0), (0.0, 0.288_675_13)],
            &[(0, 0, EdgeKind::HalfTurn), (1, 2, EdgeKind::Rotate(3))],
            3,
        )
        .with_ih(39)
    }

    /// Kites with 6-way rotation (C3C3C6C6).
    pub fn quad6() -> Self {
        from_polygon(
            &[(0.0, 0.0), (1.0, 0.0), (1.5, H), (0.0, 1.732_050_8)],
            &[(0, 1, EdgeKind::Rotate(3)), (2, 3, EdgeKind::Rotate(6))],
            3,
        )
        .with_ih(31)
    }

    /// Pentagons with 6-way rotation (CC3C3C6C6).
    pub fn pent6() -> Self {
        // Base polygon and edge pairing from ih21 (tactile-rs).
        from_polygon(
            &[
                (1.0, 0.0),
                (0.75, 0.433_013),
                (0.25, 0.433_013),
                (0.0, 0.0),
                (0.5, -0.866_025),
            ],
            &[
                (0, 0, EdgeKind::HalfTurn),
                (1, 2, EdgeKind::Rotate(3)),
                (3, 4, EdgeKind::Rotate(6)),
            ],
            3,
        )
        .with_ih(21)
    }

    // -------------------------------------------------------------- mirrors
    /// Rhombuses, with inner mirror (TT|TT).
    pub fn quad1sym() -> Self {
        from_polygon_with_mirrors(
            &[(0.0, 0.0), (1.0, 0.0), (1.5, H), (0.5, H)],
            &[(0, 2, EdgeKind::Translate), (1, 3, EdgeKind::Translate)],
            &[((0.0, 0.0), 30.0)],
            2,
        )
        .with_ih(68)
    }

    /// Hexagons, with inner mirror (tTTtTT).
    pub fn hex1sym() -> Self {
        from_polygon_with_mirrors(
            &hexagon(),
            &[
                (0, 3, EdgeKind::Translate),
                (1, 4, EdgeKind::Translate),
                (2, 5, EdgeKind::Translate),
            ],
            &[((0.75, H / 2.0), 30.0)],
            3,
        )
        .with_ih(12)
    }

    /// Rectangles (with inner mirror), rotated (tCtC).
    pub fn quad2edgesym() -> Self {
        from_polygon_with_mirrors(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 0.7), (0.0, 0.7)],
            &[
                (0, 2, EdgeKind::Translate),
                (1, 1, EdgeKind::HalfTurn),
                (3, 3, EdgeKind::HalfTurn),
            ],
            &[((0.5, 0.0), 90.0)],
            2,
        )
        .with_ih(66)
    }

    /// Kites (with inner mirror), rotated (CC|CC).
    pub fn quad2tipsym() -> Self {
        // Base polygon and edge pairing from ih69 (tactile-rs).
        from_polygon_with_mirrors(
            &[(0.5, -0.5), (1.0, 0.0), (0.5, 0.5), (0.0, 0.0)],
            &[(0, 1, EdgeKind::Rotate(4)), (2, 3, EdgeKind::Rotate(4))],
            &[((0.5, 0.5), 90.0)],
            2,
        )
        .with_ih(69)
    }

    /// Hexagons (with inner mirror), rotated (tCCtCC).
    pub fn hex2sym() -> Self {
        from_polygon_with_mirrors(
            &hexagon(),
            &[
                (0, 3, EdgeKind::Translate),
                (1, 1, EdgeKind::HalfTurn),
                (2, 2, EdgeKind::HalfTurn),
                (4, 4, EdgeKind::HalfTurn),
                (5, 5, EdgeKind::HalfTurn),
            ],
            &[((0.75, H / 2.0), 30.0)],
            3,
        )
        .with_ih(13)
    }

    /// Rhombuses, with inner mirror and 3-way rotation (C3C3|C3C3).
    pub fn quad3sym() -> Self {
        from_polygon_with_mirrors(
            &[(0.0, 0.0), (1.0, 0.0), (1.5, H), (0.5, H)],
            &[(0, 1, EdgeKind::Rotate(3)), (2, 3, EdgeKind::Rotate(3))],
            &[((1.5, H), 210.0)],
            3,
        )
        .with_ih(36)
    }

    /// Squares, with inner mirror and 4-way rotation (C4C4|C4C4).
    pub fn quad4sym() -> Self {
        from_polygon_with_mirrors(
            &unit_square(),
            &[(0, 1, EdgeKind::Rotate(4)), (2, 3, EdgeKind::Rotate(4))],
            &[((1.0, 1.0), 225.0)],
            2,
        )
        .with_ih(71)
    }
}
