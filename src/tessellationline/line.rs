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

pub fn distance(p: Point) -> f32 {
    return (p.x * p.x + p.y * p.y).sqrt();
}

pub fn breakline(p1: Point, p2: Point, current: Point, rectsize: f32) -> bool {
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
