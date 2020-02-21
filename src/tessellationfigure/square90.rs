use crate::tessellationfigure::TessellationFigure;
use crate::tessellationline::{Point, TessellationLine};
use crate::tessellationshape::TessellationShape;

impl TessellationFigure {
    pub fn square90() -> Self {
        let mut f: TessellationFigure = TessellationFigure::new();
        f.gridincx = 2.0;
        f.gridincy = 2.0;
        f.rotdiv = 4;
        f.is_reversed = true;
        f.shiftx = 0.0;
        f.shifty = 0.0;
        f.shape = TessellationShape::S;

        let mut l1: TessellationLine = TessellationLine::new(0.0, 0.0, -270.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(0.0, 1.0));
        f.append(l1);

        let mut l2: TessellationLine = TessellationLine::new(2.0, 0.0, -90.0);
        l2.append(Point::new(0.0, 1.0));
        l2.append(Point::new(1.0, 1.0));
        f.append(l2);

        f
    }
}
