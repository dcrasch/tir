use crate::tessellationfigure::TessellationFigure;
use crate::tessellationline::{Point, TessellationLine};

impl TessellationFigure {
    pub fn square() -> Self {
        let mut f: TessellationFigure = TessellationFigure::new();
        f.gridincx = 1.0;
        f.gridincy = 1.0;
        f.rotdiv = 1;
        f.sequence = 0;
        f.shiftx = 0.0;
        f.shifty = 1.0;

        let mut l1: TessellationLine = TessellationLine::new(1.0, 0.0, 0.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(0.0, 1.0));
        f.append(l1);

        let mut l2: TessellationLine = TessellationLine::new(0.0, -1.0, 0.0);
        l2.append(Point::new(0.0, 1.0));
        l2.append(Point::new(1.0, 0.1));
        f.append(l2);

        f
    }
}
