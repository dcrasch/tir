use crate::tessellationfigure::{TessellationFigure};
use crate::tessellationline::{Point};

impl IntoInterator for TessellationFigure {
    type Item = Point;

    fn into_iter(self) -> Self::IntoIter {
    }
}

impl Iterator for TessellationFigureIntoIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
    }
}

        
    
