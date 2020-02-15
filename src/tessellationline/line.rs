use euclid::Angle;

pub type Point = euclid::default::Point2D<f32>;
pub type Transform = euclid::default::Transform2D<f32>;
pub type Rotation = euclid::default::Rotation2D<f32>;

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
        let transform = Transform::create_translation(tx,ty)
            .post_rotate(Angle::degrees(angle));
        Self {
            points: Vec::<Point>::new(),
            transform,
            ci: transform.inverse().unwrap(),
            angle: angle,
            tx: tx,
            ty: ty,
        }
    }

    pub fn append(&mut self, p: Point) {
        self.points.push(p);
    }

    pub fn dpoints(&self) -> Vec<Point> {
        self.points.to_vec()
    }

    pub fn cpoints(&self) -> Vec<Point> {
        // maybe create copy of transform?
        self.points
            .iter()
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
