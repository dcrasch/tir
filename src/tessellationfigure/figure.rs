use std::iter;

use crate::tessellationline::{Point, TessellationLine};

#[derive(Default, Debug)]
pub struct TessellationFigure {
    pub lines: Vec<TessellationLine>,
    pub gridincx: f32,
    pub gridincy: f32,
    pub shiftx: f32,
    pub shifty: f32,
    pub sequence: u32,
    pub rotdiv: u32,
}

impl TessellationFigure {
    pub fn new() -> Self {
        Self {
            lines: Vec::<TessellationLine>::new(),
            gridincx: 0.0,
            gridincy: 0.0,
            shiftx: 0.0,
            shifty: 0.0,
            sequence: 0,
            rotdiv: 0,
        }
    }

    pub fn append(&mut self, l: TessellationLine) {
        self.lines.push(l);
    }

    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        self.lines.first().unwrap().dpoints()
    }
}
