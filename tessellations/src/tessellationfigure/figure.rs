use crate::tessellationfigure::coloring::{color_table, ColorTable};
use crate::tessellationfigure::colormap::color_map;
use crate::tessellationline::{Point, PointIndexPath, TessellationLine, Transform};
use crate::tessellationshape::TessellationShape;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Links one edge of the tile to another edge of the same tile, by the isometry
/// that takes the neighbouring tile across that edge. `bad` edges are related by
/// translation, rotation, glide reflection or an internal mirror.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EdgeLink {
    /// Source edge.
    pub a: usize,
    /// Edge the isometry takes edge `a` onto (traversed the other way round).
    pub b: usize,
    /// True for an internal mirror: a symmetry of the tile, not a neighbour.
    pub symmetry: bool,
    #[serde(
        serialize_with = "crate::tessellationline::transform_serialize",
        deserialize_with = "crate::tessellationline::transform_deserialize"
    )]
    pub transform: Transform,
}

impl EdgeLink {
    pub fn new(a: usize, b: usize, transform: Transform) -> Self {
        Self {
            a,
            b,
            symmetry: false,
            transform,
        }
    }
}

/// One copy of the tile in the plane.
#[derive(Clone, Copy, Debug)]
pub struct Placement {
    pub transform: Transform,
    /// Index of the tile orientation, used to pick a colour.
    pub orientation: usize,
    /// Palette index for this tile.
    pub color: usize,
}

/// Figure type with lines
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TessellationFigure {
    pub lines: Vec<TessellationLine>,
    pub links: Vec<EdgeLink>,
    /// Number of colours used to draw the tiling.
    pub colors: usize,
    pub shape: TessellationShape,
    /// Isohedral tiling type, when the figure is one of the tiled.art types.
    #[serde(default)]
    pub ih: Option<u8>,
}

impl TessellationFigure {
    pub fn new() -> Self {
        Self {
            lines: Vec::<TessellationLine>::new(),
            links: Vec::<EdgeLink>::new(),
            colors: 4,
            shape: TessellationShape::S,
            ih: None,
        }
    }

    /// Record which isohedral tiling type this figure is, so its tiles can be
    /// coloured with the matching tactile-rs table.
    pub fn with_ih(mut self, ih: u8) -> Self {
        self.ih = Some(ih);
        self
    }

    /// Append `line` to
    pub fn append(&mut self, line: TessellationLine) {
        self.lines.push(line);
    }

    /// Add a link between two edges.
    pub fn link(&mut self, a: usize, b: usize, transform: Transform) {
        self.links.push(EdgeLink::new(a, b, transform));
    }

    /// Returns the outline of the tile: the edge lines in order.
    pub fn points(&self) -> Vec<Point> {
        self.lines
            .iter()
            .flat_map(|l| l.points().to_vec())
            .collect()
    }

    /// Number of edges of the tile.
    pub fn edge_count(&self) -> usize {
        self.lines.len()
    }

    /// Check if a point falls on a line within rectsize of all the lines
    pub fn hitline(&self, point: Point, rectsize: f32) -> Option<PointIndexPath> {
        for (i, line) in self.lines.iter().enumerate() {
            if let Some(x) = line.hitline(point, rectsize) {
                return Some(PointIndexPath {
                    line_index: i,
                    point_index: x.point_index,
                    corrp: x.corrp,
                });
            }
        }
        None
    }

    /// Check if a point falls on a point within rectsize of all the lines
    pub fn hitpoints(&self, point: Point, rectsize: f32) -> Option<PointIndexPath> {
        for (i, line) in self.lines.iter().enumerate() {
            if let Some(x) = line.hitpoint(point, rectsize) {
                return Some(PointIndexPath {
                    line_index: i,
                    point_index: x.point_index,
                    corrp: x.corrp,
                });
            }
        }
        None
    }

    /// Insert a `point` after `point_index_path`, together with the points it
    /// corresponds to on every linked edge, so paired and mirrored edges keep
    /// matching.
    pub fn insert(&mut self, point_index_path: PointIndexPath, point: Point) {
        let p1 = if point_index_path.corrp {
            self.lines[point_index_path.line_index].cpoint(point)
        } else {
            point
        };
        let line = point_index_path.line_index;
        let index = point_index_path.point_index + 1;
        self.lines[line].insert(index, p1);
        self.propagate_insert(line, index);
    }

    /// Copy a freshly inserted point onto the linked edges, and from there onto
    /// their links, until every edge carries the matching point.
    fn propagate_insert(&mut self, line: usize, index: usize) {
        let links = self.link_info();
        let mut queue = vec![(line, index)];
        let mut inserted = 0usize;
        while let Some((l, i)) = queue.pop() {
            if i >= self.lines[l].points().len() {
                continue;
            }
            let p = self.lines[l].points()[i];
            for (to, t) in linked(&links, l) {
                let q = t.transform_point(p);
                // The linked edge may already carry this point.
                if self.point_within(to, q, INSERT_TOL).is_some() {
                    continue;
                }
                if inserted > 4096 {
                    return;
                }
                let j = self.lines[to].insertion_position(q);
                self.lines[to].insert(j, q);
                inserted += 1;
                queue.push((to, j));
            }
        }
    }

    /// Update a `point` at `point_index_path`, and all points linked to it.
    pub fn update(&mut self, point_index_path: PointIndexPath, point: Point) {
        let p1 = if point_index_path.corrp {
            self.lines[point_index_path.line_index].cpoint(point)
        } else {
            point
        };
        let line = point_index_path.line_index;
        let index = point_index_path.point_index;
        if index >= self.lines[line].points().len() {
            return;
        }
        // A half turn or inner mirror that fixes the point (the point lies on
        // its axis) hides the point's image inside itself. As soon as the point
        // leaves the axis the image is a second point of the same edge.
        let old = self.lines[line].points()[index];
        let splits: Vec<Transform> = {
            let links = self.link_info();
            linked(&links, line)
                .filter(|&(to, t)| {
                    to == line
                        && (t.transform_point(old) - old).square_length() <= EDIT_TOL * EDIT_TOL
                        && (t.transform_point(p1) - p1).square_length() > EDIT_TOL * EDIT_TOL
                        // A previous crossing may already have left a second
                        // point at the old position.
                        && self
                            .point_within_except(line, old, EDIT_TOL, Some(index))
                            .is_none()
                })
                .map(|(_, t)| t)
                .collect()
        };
        self.set_point(line, index, p1);
        for t in splits {
            let q = t.transform_point(p1);
            let pos = self.lines[line].insertion_position(q);
            self.lines[line].insert(pos, q);
            self.propagate_insert(line, pos);
        }
    }

    /// Move one point and propagate through the edge links, so the tiling stays
    /// consistent. Linked points are identified by their position before the
    /// move, which is exact because every earlier edit moved them together.
    fn set_point(&mut self, line: usize, index: usize, point: Point) {
        let links = self.link_info();
        let mut queue = vec![(line, index, Transform::identity())];
        let mut moves: Vec<(usize, usize, Transform)> = Vec::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        while let Some((l, i, acc)) = queue.pop() {
            if !seen.insert((l, i)) {
                continue;
            }
            if i >= self.lines[l].points().len() {
                continue;
            }
            moves.push((l, i, acc));
            let p = self.lines[l].points()[i];
            for (to, t) in linked(&links, l) {
                let q = t.transform_point(p);
                // On a self-link the point can be its own image (on the axis);
                // then the real partner is a second coincident point, if any.
                let except = (to == l).then_some(i);
                if let Some(j) = self.point_within_except(to, q, EDIT_TOL, except) {
                    if !seen.contains(&(to, j)) {
                        queue.push((to, j, acc.then(&t)));
                    }
                }
            }
        }
        for (l, i, acc) in moves {
            self.lines[l].update(i, acc.transform_point(point));
        }
    }

    /// Index of the point closest to `p`, if it is within `tol`.
    fn point_within(&self, line: usize, p: Point, tol: f32) -> Option<usize> {
        self.point_within_except(line, p, tol, None)
    }

    /// Index of the closest point to `p` within `tol`, ignoring `except`.
    fn point_within_except(
        &self,
        line: usize,
        p: Point,
        tol: f32,
        except: Option<usize>,
    ) -> Option<usize> {
        let mut best: Option<(usize, f32)> = None;
        for (i, &q) in self.lines[line].points().iter().enumerate() {
            if Some(i) == except {
                continue;
            }
            let d = (q - p).square_length();
            if d <= tol * tol && best.is_none_or(|(_, bd)| d < bd) {
                best = Some((i, d));
            }
        }
        best.map(|(i, _)| i)
    }

    /// The edges and isometries linking them.
    fn link_info(&self) -> Vec<(usize, usize, Transform)> {
        self.links
            .iter()
            .map(|link| (link.a, link.b, link.transform))
            .collect()
    }

    /// Copies of the tile that cover a `width` x `height` area centred on the
    /// origin, walked through the edge links.
    pub fn placements(&self, width: f32, height: f32) -> Vec<Placement> {
        let poly = self.points();
        if poly.is_empty() {
            return Vec::new();
        }
        let (xmin, xmax, ymin, ymax) = bounds(&poly, &Transform::identity());
        let margin = 2.0 * (xmax - xmin).max(ymax - ymin).max(0.1);
        let xmin = -width / 2.0 - margin;
        let xmax = width / 2.0 + margin;
        let ymin = -height / 2.0 - margin;
        let ymax = height / 2.0 + margin;

        let mut result: Vec<Placement> = Vec::new();
        let mut seen: HashMap<Cell, Vec<Transform>> = HashMap::new();
        let mut total = 0usize;
        let mut queue = vec![Transform::identity()];
        while let Some(t) = queue.pop() {
            if total > 5000 {
                break;
            }
            // Look for an equal placement in the 3^2 neighbouring cells. The
            // linear part is one of a handful of orientations per translation,
            // so keying by the translation is enough and compares far fewer
            // cells than a six-dimensional key.
            let c = cell(&t);
            let mut duplicate = false;
            'search: for i in 0..9 {
                let mut k = c;
                let mut r = i;
                for slot in &mut k {
                    *slot += (r % 3) as i64 - 1;
                    r /= 3;
                }
                if let Some(ts) = seen.get(&k) {
                    for u in ts {
                        if close(u, &t) {
                            duplicate = true;
                            break 'search;
                        }
                    }
                }
            }
            if duplicate {
                continue;
            }
            seen.entry(c).or_default().push(t);
            total += 1;
            let (txmin, txmax, tymin, tymax) = bounds(&poly, &t);
            if txmax < xmin || txmin > xmax || tymax < ymin || tymin > ymax {
                continue;
            }
            result.push(Placement {
                transform: t,
                orientation: 0,
                color: 0,
            });
            for link in &self.links {
                if link.symmetry {
                    continue;
                }
                for g in [link.transform, link.transform.inverse().unwrap()] {
                    queue.push(t.then(&g));
                }
            }
        }

        // Number the tile orientations in order of first appearance and pick
        // colours from them.
        let mut keys: Vec<Orientation> = Vec::new();
        for p in &result {
            let k = orientation_key(&p.transform);
            if !keys.contains(&k) {
                keys.push(k);
            }
        }
        for p in &mut result {
            p.orientation = keys
                .iter()
                .position(|&k| k == orientation_key(&p.transform))
                .unwrap_or(0);
            p.color = p.orientation % self.colors.max(1);
        }

        // Use the tactile-rs colouring for this tiling type, if known. The
        // centroid of each tile identifies its aspect and its integer
        // coordinates on the translation lattice, which stays unambiguous even
        // for prototiles with internal symmetries.
        let base = self
            .lines
            .iter()
            .filter_map(|line| line.points().first().copied())
            .fold((0.0f32, 0.0f32, 0usize), |(x, y, n), p| {
                (x + p.x, y + p.y, n + 1)
            });
        let base = if base.2 == 0 {
            Point::origin()
        } else {
            Point::new(base.0 / base.2 as f32, base.1 / base.2 as f32)
        };
        let mut mapped = false;
        if let (Some(table), Some(map)) =
            (self.ih.and_then(color_table), self.ih.and_then(color_map))
        {
            mapped = result.iter_mut().all(|p| {
                if let Some(color) = map.color(p, table, base) {
                    p.color = color;
                    true
                } else {
                    false
                }
            });
        }

        // Fall back to the orientation/lattice approximation for figures
        // without tactile data, or if an edited tile no longer matches it.
        if !mapped {
            if let Some(table) = self.ih.and_then(color_table) {
                color_by_lattice(&mut result, table);
            } else if result.len() > 1
                && result
                    .iter()
                    .all(|p| p.orientation == result[0].orientation)
            {
                // Translation-only tilings have a single orientation; colour
                // them by lattice parity so neighbouring tiles differ.
                if let Some((t1, t2)) = lattice(&result) {
                    let det = t1.0 * t2.1 - t1.1 * t2.0;
                    let n = self.colors.max(1) as f32;
                    for p in &mut result {
                        let (x, y) = (p.transform.m31, p.transform.m32);
                        let i = (x * t2.1 - y * t2.0) / det;
                        let j = (-x * t1.1 + y * t1.0) / det;
                        p.color = ((i + j).round().rem_euclid(n)) as usize;
                    }
                }
            }
        }
        result
    }

    pub fn load(&mut self, other: Self) {
        self.lines = other.lines;
        self.links = other.links;
        self.colors = other.colors;
        self.shape = other.shape;
        self.ih = other.ih;
    }
}

/// Tolerance for finding a point that a link was just paired with.
const INSERT_TOL: f32 = 1e-4;
/// Tolerance for finding the counterpart of a point being dragged.
const EDIT_TOL: f32 = 1e-4;

/// The edges linked to `line`, with the isometry onto each.
fn linked<'a>(
    links: &'a [(usize, usize, Transform)],
    line: usize,
) -> impl Iterator<Item = (usize, Transform)> + 'a {
    links.iter().filter_map(move |&(a, b, t)| {
        if a == line {
            Some((b, t))
        } else if b == line {
            t.inverse().map(|inv| (a, inv))
        } else {
            None
        }
    })
}

/// Coarse cell of a translation, for de-duplicating placements. Composing many
/// transforms accumulates float drift, so equal placements can differ by more
/// than a fine rounding grid; neighbours within one cell are compared instead.
type Cell = [i64; 2];

const CELL_SIZE: f32 = 0.05;

fn cell(t: &Transform) -> Cell {
    let r = |v: f32| (v / CELL_SIZE).floor() as i64;
    [r(t.m31), r(t.m32)]
}

fn close(a: &Transform, b: &Transform) -> bool {
    const TOL: f32 = 2e-3;
    (a.m11 - b.m11).abs() < TOL
        && (a.m12 - b.m12).abs() < TOL
        && (a.m21 - b.m21).abs() < TOL
        && (a.m22 - b.m22).abs() < TOL
        && (a.m31 - b.m31).abs() < TOL
        && (a.m32 - b.m32).abs() < TOL
}

/// Rotation angle (in 1/720 turn) and flip flag of a placement.
type Orientation = (usize, bool);

fn orientation_key(t: &Transform) -> Orientation {
    let det = t.m11 * t.m22 - t.m12 * t.m21;
    let steps = if det >= 0.0 {
        let angle = t.m12.atan2(t.m11);
        (angle.rem_euclid(std::f32::consts::TAU) / (std::f32::consts::TAU / 720.0)).round() as usize
            % 720
    } else {
        // A reflection is unchanged by adding half a turn to its axis.
        let angle = 0.5 * t.m12.atan2(t.m11);
        (angle.rem_euclid(std::f32::consts::PI) / (std::f32::consts::PI / 360.0)).round() as usize
            % 360
    };
    (steps, det < 0.0)
}

/// Colour every placement from the tactile-rs table for its tiling type.
///
/// The table colours a tile from its aspect and its integer coordinates on the
/// tiling's translation lattice. Coordinates are found by dividing each
/// placement by the representative of its orientation: what remains is a
/// lattice translation, whose coordinates are read off the lattice basis.
fn color_by_lattice(placements: &mut [Placement], table: &ColorTable) {
    let translations: Vec<(f32, f32)> = placements
        .iter()
        .filter(|p| is_translation(&p.transform))
        .map(|p| (p.transform.m31, p.transform.m32))
        .collect();
    let Some((t1, t2)) = lattice_basis(&translations) else {
        return;
    };
    let det = t1.0 * t2.1 - t1.1 * t2.0;

    // One representative placement per orientation: the one nearest the
    // figure's origin, which is the analogue of the aspect placed in the
    // central lattice cell.
    let mut aspects: Vec<Transform> = Vec::new();
    let mut nearest: Vec<f32> = Vec::new();
    for p in placements.iter() {
        let d = p.transform.m31 * p.transform.m31 + p.transform.m32 * p.transform.m32;
        if p.orientation >= aspects.len() {
            aspects.push(p.transform);
            nearest.push(d);
        } else if d < nearest[p.orientation] {
            aspects[p.orientation] = p.transform;
            nearest[p.orientation] = d;
        }
    }

    for p in placements.iter_mut() {
        let Some(&aspect) = aspects.get(p.orientation) else {
            continue;
        };
        let Some(inverse) = aspect.inverse() else {
            continue;
        };
        // Aspect first, then the placement, leaves only a lattice step.
        let step = inverse.then(&p.transform);
        let (x, y) = (step.m31, step.m32);
        let a = (x * t2.1 - y * t2.0) / det;
        let b = (t1.0 * y - t1.1 * x) / det;
        let (ai, bi) = (a.round(), b.round());
        let dx = ai * t1.0 + bi * t2.0 - x;
        let dy = ai * t1.1 + bi * t2.1 - y;
        if dx * dx + dy * dy < 1e-2 {
            p.color = table.color(p.orientation, ai as i64, bi as i64);
        }
    }
}

/// A basis of the translation lattice: the shortest vector, paired with the
/// shortest vector that together generate every given translation.
fn lattice_basis(translations: &[(f32, f32)]) -> Option<((f32, f32), (f32, f32))> {
    let len2 = |t: (f32, f32)| t.0 * t.0 + t.1 * t.1;
    let cross = |a: (f32, f32), b: (f32, f32)| a.0 * b.1 - a.1 * b.0;
    let mut candidates: Vec<(f32, f32)> = translations
        .iter()
        .copied()
        .filter(|t| len2(*t) > 1e-6)
        .collect();
    candidates.sort_by(|a, b| len2(*a).partial_cmp(&len2(*b)).unwrap());
    candidates.dedup_by(|a, b| len2((a.0 - b.0, a.1 - b.1)) < 1e-6);

    for (i, &t1) in candidates.iter().enumerate() {
        for (j, &t2) in candidates.iter().enumerate() {
            if i == j {
                continue;
            }
            let det = cross(t1, t2);
            if det.abs() < 1e-3 {
                continue;
            }
            let generates_all = translations.iter().all(|&v| {
                let a = cross(v, t2) / det;
                let b = cross(t1, v) / det;
                (a - a.round()).abs() < 0.01 && (b - b.round()).abs() < 0.01
            });
            if generates_all {
                return Some((t1, t2));
            }
        }
    }
    None
}

/// True if `t` is a pure translation.
fn is_translation(t: &Transform) -> bool {
    (t.m11 - 1.0).abs() < 1e-4
        && t.m12.abs() < 1e-4
        && t.m21.abs() < 1e-4
        && (t.m22 - 1.0).abs() < 1e-4
}

/// Two shortest independent translation vectors of a set of placements.
fn lattice(placements: &[Placement]) -> Option<((f32, f32), (f32, f32))> {
    let mut ts: Vec<(f32, f32)> = placements
        .iter()
        .map(|p| (p.transform.m31, p.transform.m32))
        .filter(|t| t.0 * t.0 + t.1 * t.1 > 1e-6)
        .collect();
    ts.sort_by(|a, b| {
        let da = a.0 * a.0 + a.1 * a.1;
        let db = b.0 * b.0 + b.1 * b.1;
        da.partial_cmp(&db).unwrap()
    });
    let t1 = *ts.first()?;
    let t2 = *ts.iter().find(|t| (t1.0 * t.1 - t1.1 * t.0).abs() > 1e-3)?;
    Some((t1, t2))
}

/// Axis aligned bounds of `poly` after `t`.
fn bounds(poly: &[Point], t: &Transform) -> (f32, f32, f32, f32) {
    let mut xmin = f32::MAX;
    let mut xmax = f32::MIN;
    let mut ymin = f32::MAX;
    let mut ymax = f32::MIN;
    for &p in poly {
        let q = t.transform_point(p);
        xmin = xmin.min(q.x);
        xmax = xmax.max(q.x);
        ymin = ymin.min(q.y);
        ymax = ymax.max(q.y);
    }
    (xmin, xmax, ymin, ymax)
}
