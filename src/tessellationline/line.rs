use nalgebra::{Matrix3, Point2, UnitComplex, Vector2};

pub type Point = Point2<f32>;
pub type Matrix = Matrix3<f32>;

#[derive(Debug)]
pub struct TessellationLine {
    pub points: Vec<Point>,
    transform: Matrix,
    ci: Matrix,
    pub angle: f32,
    pub tx: f32,
    pub ty: f32,
}

impl TessellationLine {
    pub fn new(tx: f32, ty: f32, angle: f32) -> Self {
        // https://www.nalgebra.org/points_and_transformations/
        let transform: Matrix = Matrix::identity().append_translation(&Vector2::<f32>::new(tx, ty))
            * UnitComplex::new(angle / 180.0 * std::f32::consts::PI).to_homogeneous();
        Self {
            points: Vec::<Point2<f32>>::new(),
            transform,
            ci: transform.try_inverse().unwrap(),
            angle: 0.0,
            tx: 0.0,
            ty: 0.0,
        }
    }
    pub fn append(&mut self, p: Point) {
        self.points.push(p);
    }
}

impl Default for TessellationLine {
    fn default() -> Self {
        Self {
            points: Vec::default(),
            transform: Matrix::identity(),
            ci: Matrix::identity(),
            angle: 0.0,
            tx: 0.0,
            ty: 0.0
        }
    }
}
