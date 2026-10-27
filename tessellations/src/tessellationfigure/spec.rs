use crate::tessellationfigure::{EdgeLink, TessellationFigure};
use crate::tessellationline::{Point, TessellationLine, Transform};

/// How an edge is related to its twin edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeKind {
    /// Translated onto the twin edge.
    Translate,
    /// Related to itself by a half turn about its midpoint.
    HalfTurn,
    /// Rotated about the shared vertex by 360/`n` degrees.
    Rotate(u32),
    /// Glide reflected onto the twin edge.
    Glide,
}

/// Edge pairing: edge `a` is taken onto edge `b`.
pub type Pairing = (usize, usize, EdgeKind);

/// A mirror axis: a point on the line and its direction in degrees. The tile is
/// symmetric across this line.
pub type MirrorAxis = ((f32, f32), f32);

fn complex_div((ax, ay): (f32, f32), (bx, by): (f32, f32)) -> (f32, f32) {
    let d = bx * bx + by * by;
    ((ax * bx + ay * by) / d, (ay * bx - ax * by) / d)
}

/// The isometry taking the segment `p1 -> p2` onto `q1 -> q2`.
/// `direct` picks rotation/translation, otherwise reflection/glide.
fn isometry(p1: Point, p2: Point, q1: Point, q2: Point, direct: bool) -> Transform {
    let d = (p2.x - p1.x, p2.y - p1.y);
    let e = (q2.x - q1.x, q2.y - q1.y);
    let c = if direct {
        complex_div(e, d)
    } else {
        complex_div(e, (d.0, -d.1))
    };
    let (a, b) = c;
    if direct {
        let tx = q1.x - (a * p1.x - b * p1.y);
        let ty = q1.y - (b * p1.x + a * p1.y);
        Transform::new(a, b, -b, a, tx, ty)
    } else {
        let tx = q1.x - (a * p1.x + b * p1.y);
        let ty = q1.y - (b * p1.x - a * p1.y);
        Transform::new(a, b, b, -a, tx, ty)
    }
}

/// Reflection in the line through `p` with direction `degrees`.
fn reflection(p: (f32, f32), degrees: f32) -> Transform {
    let a = 2.0 * degrees.to_radians();
    let (c, s) = (a.cos(), a.sin());
    let (x, y) = p;
    Transform::new(c, s, s, -c, x - c * x - s * y, y - s * x + c * y)
}

/// Build a figure from a polygon and its edge pairing.
pub fn from_polygon(
    vertices: &[(f32, f32)],
    pairings: &[Pairing],
    colors: usize,
) -> TessellationFigure {
    from_polygon_with_mirrors(vertices, pairings, &[], colors)
}

/// Build a figure from a polygon, its edge pairing and its inner mirror axes.
pub fn from_polygon_with_mirrors(
    vertices: &[(f32, f32)],
    pairings: &[Pairing],
    mirrors: &[MirrorAxis],
    colors: usize,
) -> TessellationFigure {
    let v: Vec<Point> = vertices.iter().map(|&(x, y)| Point::new(x, y)).collect();
    let n = v.len();
    let mut figure = TessellationFigure::new();
    figure.colors = colors;
    for i in 0..n {
        let mut line = TessellationLine::from_transform(Transform::identity());
        line.append(v[i]);
        line.append(v[(i + 1) % n]);
        figure.append(line);
    }
    for &(a, b, kind) in pairings {
        let p1 = v[a];
        let p2 = v[(a + 1) % n];
        // Direct isometries reverse the endpoint order, opposite ones keep it.
        let (q1, q2, direct) = match kind {
            EdgeKind::Translate | EdgeKind::HalfTurn | EdgeKind::Rotate(_) => {
                (v[(b + 1) % n], v[b], true)
            }
            EdgeKind::Glide => (v[b], v[(b + 1) % n], false),
        };
        figure
            .links
            .push(EdgeLink::new(a, b, isometry(p1, p2, q1, q2, direct)));
    }
    // Inner mirrors are symmetries of the tile itself: they never create new
    // tiles, but keep edited points consistent.
    for &(point, degrees) in mirrors {
        let m = reflection(point, degrees);
        for i in 0..n {
            let (a, b) = (m.transform_point(v[i]), m.transform_point(v[(i + 1) % n]));
            let j = (0..n).find(|&j| {
                let (c, d) = (v[j], v[(j + 1) % n]);
                let eq = |p: Point, q: Point| (p - q).square_length() < 1e-6;
                (eq(a, c) && eq(b, d)) || (eq(a, d) && eq(b, c))
            });
            if let Some(j) = j {
                let mut link = EdgeLink::new(i, j, m);
                link.symmetry = true;
                figure.links.push(link);
            }
        }
    }
    figure
}

fn pt(x: f32, y: f32) -> (f32, f32) {
    (x, y)
}

const SQRT3: f32 = 0.866_025_4;

impl TessellationFigure {
    /// Parallelograms: one orientation, translated edges (Heesch TTTT).
    pub fn square() -> Self {
        from_polygon(
            &[pt(0.0, 0.0), pt(1.0, 0.0), pt(1.0, 1.0), pt(0.0, 1.0)],
            &[(0, 2, EdgeKind::Translate), (1, 3, EdgeKind::Translate)],
            2,
        )
        .with_ih(41)
    }

    /// Hexagons: one orientation (TTTTTT).
    pub fn hexagon() -> Self {
        from_polygon(
            &[
                pt(1.0, 0.0),
                pt(0.5, SQRT3),
                pt(-0.5, SQRT3),
                pt(-1.0, 0.0),
                pt(-0.5, -SQRT3),
                pt(0.5, -SQRT3),
            ],
            &[
                (0, 3, EdgeKind::Translate),
                (1, 4, EdgeKind::Translate),
                (2, 5, EdgeKind::Translate),
            ],
            3,
        )
        .with_ih(1)
    }

    /// Triangles, rotated: two orientations (CCC).
    pub fn triangle() -> Self {
        from_polygon(
            &[pt(0.0, 0.0), pt(1.0, 0.0), pt(0.5, SQRT3)],
            &[
                (0, 0, EdgeKind::HalfTurn),
                (1, 1, EdgeKind::HalfTurn),
                (2, 2, EdgeKind::HalfTurn),
            ],
            2,
        )
        .with_ih(84)
    }

    /// Squares with 4-way rotation (C4C4C4C4).
    pub fn square90() -> Self {
        from_polygon(
            &[pt(0.0, 0.0), pt(1.0, 0.0), pt(1.0, 1.0), pt(0.0, 1.0)],
            &[(0, 1, EdgeKind::Rotate(4)), (2, 3, EdgeKind::Rotate(4))],
            2,
        )
        .with_ih(55)
    }

    /// Rhombuses with 3-way rotation (C3C3C3C3).
    pub fn diamond() -> Self {
        from_polygon(
            &[pt(0.0, 0.0), pt(1.0, 0.0), pt(1.5, SQRT3), pt(0.5, SQRT3)],
            &[(0, 1, EdgeKind::Rotate(3)), (2, 3, EdgeKind::Rotate(3))],
            3,
        )
        .with_ih(33)
    }
}
