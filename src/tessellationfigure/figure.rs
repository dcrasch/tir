use crate::tessellationline::{Point, PointIndexPath, TessellationLine};
use crate::tessellationshape::TessellationShape;

#[derive(Default, Debug)]
pub struct TessellationFigure {
    pub lines: Vec<TessellationLine>,
    pub gridincx: f32,
    pub gridincy: f32,
    pub shiftx: f32,
    pub shifty: f32,
    pub rotdiv: u32,
    pub is_reversed: bool,
    pub shape: TessellationShape,
}

impl TessellationFigure {
    pub fn new() -> Self {
        Self {
            lines: Vec::<TessellationLine>::new(),
            gridincx: 0.0,
            gridincy: 0.0,
            shiftx: 0.0,
            shifty: 0.0,
            is_reversed: false, // not per line??
            shape: TessellationShape::S,
            rotdiv: 0,
        }
    }

    /// Append `line` to
    pub fn append(&mut self, line: TessellationLine) {
        self.lines.push(line);
    }

    /// Returns a list of the points and transformed points in the order of the figure.
    pub fn points(&self) -> Vec<Point> {
        (&self.lines)
            .iter()
            .flat_map(|x| x.dpoints())
            .chain((&self.lines).iter().flat_map(|x| x.cpoints()))
            .collect()
    }

    /// Check if a point falls on a line within rectsize of all the lines
    pub fn hitline(&self, point: Point, rectsize: f32) -> Option<PointIndexPath> {
        for (i, line) in self.lines.iter().enumerate() {
            match line.hitline(point, rectsize) {
                Some(x) => {
                    return Some(PointIndexPath {
                        line_index: i,
                        point_index: x.point_index,
                        corrp: x.corrp,
                    })
                }
                _ => (),
            }
        }
        None
    }

    /// Insert a `point` at `point_index_path`
    pub fn insert(&mut self, point_index_path: PointIndexPath, point: Point) {
        let p1 = if point_index_path.corrp {
            self.lines[point_index_path.line_index].cpoint(point)
        } else {
            point
        };
        self.lines[point_index_path.line_index].insert(point_index_path.point_index, p1);
    }
}
