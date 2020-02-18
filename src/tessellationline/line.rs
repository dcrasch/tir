use euclid::Angle;

pub type Point = euclid::default::Point2D<f32>;
pub type Transform = euclid::default::Transform2D<f32>;

#[derive(Debug)]
pub struct TessellationLine {
    points: Vec<Point>,
    transform: Transform,
    ci: Transform,
    angle: f32,
    tx: f32,
    ty: f32,
}

impl TessellationLine {
    pub fn new(tx: f32, ty: f32, angle: f32) -> Self {
        let transform = Transform::create_translation(tx, ty).post_rotate(Angle::degrees(angle));
        Self {
            points: Vec::<Point>::new(),
            transform,
            ci: transform.inverse().unwrap(),
            angle,
            tx,
            ty,
        }
    }

    pub fn append(&mut self, p: Point) {
        self.points.push(p);
    }

    pub fn dpoints(&self) -> Vec<Point> {
        self.points.to_vec()
    }

    pub fn cpoints(&self) -> Vec<Point> {
        self.points
            .iter()
            .rev()
            .map(move |p| self.transform.transform_point(*p))
            .collect()
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
    return (p.x * p.x + p.y * p.y).sqrt();
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
    return false;
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
}
