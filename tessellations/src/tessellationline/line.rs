use euclid::vec2;
use euclid::Angle;

use serde::de::Deserializer;
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};

pub type Point = euclid::default::Point2D<f32>;

pub type Transform = euclid::default::Transform2D<f32>;

/// Type to store the index of a point on a line
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PointIndexPath {
    pub line_index: usize,
    pub point_index: usize,
    pub corrp: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PointDef {
    x: f32,
    y: f32,
}

fn points_serialize<S>(v: &[Point], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = s.serialize_seq(Some(v.len()))?;
    for p in v {
        seq.serialize_element(&PointDef { x: p.x, y: p.y })?;
    }
    seq.end()
}

fn points_deserialize<'de, D>(d: D) -> Result<Vec<Point>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Vec::deserialize(d)?;
    Ok(v.into_iter()
        .map(|p: PointDef| Point::new(p.x, p.y))
        .collect())
}

/// Type to store the points on a line and the transform to the corresponding line
#[derive(Debug, Serialize, Deserialize)]
pub struct TessellationLine {
    #[serde(
        serialize_with = "points_serialize",
        deserialize_with = "points_deserialize"
    )]
    points: Vec<Point>,

    #[serde(skip)]
    transform: Transform,

    #[serde(skip)]
    ci: Transform,
    angle: f32,
    tx: f32,
    ty: f32,
}

impl TessellationLine {
    pub fn new(tx: f32, ty: f32, angle: f32) -> Self {
        let transform = Transform::rotation(Angle::degrees(-angle)).then_translate(vec2(tx, ty));
        Self {
            points: Vec::<Point>::new(),
            transform,
            ci: transform.inverse().unwrap(),
            angle,
            tx,
            ty,
        }
    }

    /// Append the `point` to the back of the points
    pub fn append(&mut self, p: Point) {
        self.points.push(p);
    }

    /// Remove the point at `index` and shift down the following elements.
    pub fn remove(&mut self, index: usize) {
        self.points.remove(index);
    }

    /// Insert `point` at `position`
    pub fn insert(&mut self, index: usize, point: Point) {
        self.points.insert(index, point);
    }

    /// Update the point on a line
    pub fn update(&mut self, index: usize, point: Point) {
        self.points[index] = point;
    }

    /// get a list of the points
    pub fn dpoints(&self) -> Vec<Point> {
        self.points.to_vec()
    }

    /// get a list of the transformed points
    pub fn cpoints(&self) -> Vec<Point> {
        self.points
            .iter()
            .map(|&p| self.transform.transform_point(p))
            .collect()
    }

    /// get a list of the transformed points
    pub fn crpoints(&self) -> Vec<Point> {
        self.points
            .iter()
            .rev()
            .map(|&p| self.transform.transform_point(p))
            .collect()
    }

    /// transform a `point` using the transform matrix
    pub fn cpoint(&self, point: Point) -> Point {
        self.ci.transform_point(point)
    }

    /// check if a point
    pub fn hitpoint(&self, p1: Point, rectsize: f32) -> Option<PointIndexPath> {
        let p2 = self.ci.transform_point(p1);
        let second_last = self.points.len() - 1;
        for (i, &p) in self.points[1..second_last].iter().enumerate() {
            if hit(p1, p, rectsize) {
                return Some(PointIndexPath {
                    line_index: 0,
                    point_index: i + 1,
                    corrp: false,
                });
            }
            if hit(p2, p, rectsize) {
                return Some(PointIndexPath {
                    line_index: 0,
                    point_index: i + 1,
                    corrp: true,
                });
            }
        }
        None
    }

    /// Check if a point falls on a line within rectsize
    pub fn hitline(&self, p1: Point, rectsize: f32) -> Option<PointIndexPath> {
        let p2 = self.ci.transform_point(p1);
        for (i, p) in self.points.windows(2).enumerate() {
            if breakline(p[0], p[1], p1, rectsize) {
                return Some(PointIndexPath {
                    line_index: 0,
                    point_index: i,
                    corrp: false,
                });
            }
            if breakline(p[0], p[1], p2, rectsize) {
                return Some(PointIndexPath {
                    line_index: 0,
                    point_index: i,
                    corrp: true,
                });
            }
        }
        None
    }
}

impl Default for TessellationLine {
    fn default() -> Self {
        Self {
            points: Vec::default(),
            transform: Transform::identity(),
            ci: Transform::identity(),
            angle: 0.0,
            tx: 0.0,
            ty: 0.0,
        }
    }
}

fn hit(p1: Point, p2: Point, rectsize: f32) -> bool {
    let d: Point = (p1 - p2).to_point();
    (d.x < rectsize) && (d.x > -rectsize) && (d.y < rectsize) && (d.y > -rectsize)
}

fn distance(p: Point) -> f32 {
    (p.x * p.x + p.y * p.y).sqrt()
}

fn breakline(p1: Point, p2: Point, current: Point, rectsize: f32) -> bool {
    let d: Point = (p1 - p2).to_point();
    let r = distance(d);

    if r > 0.0 {
        let distancefromline =
            ((current.x * d.y - current.y * d.x + d.x * p2.y - d.y * p2.x) / r).abs();
        if distancefromline < rectsize
            && distance((current - p1).to_point()) < r
            && distance((current - p2).to_point()) < r
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_breakline_on() {
        assert_eq!(
            breakline(
                Point::new(0.0, 0.0),
                Point::new(1.0, 0.0),
                Point::new(0.5, 0.0),
                0.1
            ),
            true
        );
    }

    #[test]
    fn test_breakline_ontop() {
        assert_eq!(
            breakline(
                Point::new(0.0, 0.0),
                Point::new(1.0, 0.0),
                Point::new(0.5, 0.05),
                0.1
            ),
            true
        );
    }

    #[test]
    fn test_breakline_offright() {
        assert_eq!(
            breakline(
                Point::new(0.0, 0.0),
                Point::new(1.0, 0.0),
                Point::new(1.01, 0.0),
                0.1
            ),
            false
        );
    }

    #[test]
    fn test_breakline_offleft() {
        assert_eq!(
            breakline(
                Point::new(0.0, 0.0),
                Point::new(1.0, 0.0),
                Point::new(-0.05, 0.0),
                0.1
            ),
            false
        );
    }

    #[test]
    fn test_breakline_offmoreleft() {
        assert_eq!(
            breakline(
                Point::new(0.0, 0.0),
                Point::new(1.0, 0.0),
                Point::new(-0.2, 0.0),
                0.1
            ),
            false
        );
    }

    #[test]
    fn test_breakline_offmoreright() {
        assert_eq!(
            breakline(
                Point::new(0.0, 0.0),
                Point::new(1.0, 0.0),
                Point::new(1.2, 0.0),
                0.1
            ),
            false
        );
    }

    #[test]
    fn test_distance_zeropoint() {
        assert_eq!(distance(Point::new(0.0, 0.0)), 0.0);
    }

    #[test]
    fn test_distance_3and4() {
        assert_eq!(distance(Point::new(3.0, 4.0)), 5.0);
    }

    #[test]
    fn test_distance_neg3and4() {
        assert_eq!(distance(Point::new(-3.0, 4.0)), 5.0);
    }

    #[test]
    fn test_distance_neg3neg4() {
        assert_eq!(distance(Point::new(-3.0, -4.0)), 5.0);
    }

    #[test]
    fn test_hitpoint() {
        assert_eq!(hit(Point::new(0.0, 0.0), Point::new(0.0, 0.0), 5.0), true);
    }

    #[test]
    fn test_misspoint() {
        assert_eq!(hit(Point::new(0.0, 0.0), Point::new(10.0, 0.0), 5.0), false);
    }

    #[test]
    fn test_transform() {
        // let transform = Transform::create_translation(tx,ty).post_rotate(a);
    }
}
