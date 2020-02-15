use std::iter;

use crate::tessellationline::{Point, TessellationLine};

#[derive(Debug)]
pub enum TessellationShape {
    S,
    U,
    I,
    J,
}

impl Default for TessellationShape {
    fn default() -> Self {
        TessellationShape::S
    }
}

#[derive(Default, Debug)]
pub struct TessellationFigure {
    pub lines: Vec<TessellationLine>,
    pub gridincx: f32,
    pub gridincy: f32,
    pub shiftx: f32,
    pub shifty: f32,
    pub rotdiv: u32,
    pub isReversed: bool,
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
            isReversed: false, // not per line??
            shape: TessellationShape::S,
            rotdiv: 0,
        }
    }

    pub fn append(&mut self, l: TessellationLine) {
        self.lines.push(l);
    }

    pub fn points(&self) -> Vec<Point> {
        (&self.lines)
            .into_iter()
            .flat_map(|x| x.dpoints())
            .chain((&self.lines).into_iter().flat_map(|x| x.cpoints()))
            .collect()
    }
}
