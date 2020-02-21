use crate::tessellationfigure::TessellationFigure;
use crate::tessellationline::{Point, TessellationLine};
use crate::tessellationshape::TessellationShape;

impl TessellationFigure {
    pub fn hexagon() -> Self {
        let mut f: TessellationFigure = TessellationFigure::new();
        f.gridincx = 3.73205;
        f.gridincy = 0.866025;
        f.rotdiv = 1;
        f.is_reversed = false;
        f.shiftx = 1.866025;
        f.shifty = 0.0;
        f.shape = TessellationShape::S;

        let mut l1: TessellationLine = TessellationLine::new(0.0, 1.73205, 0.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(1.0, 0.0));
        f.append(l1);

        let mut l2: TessellationLine = TessellationLine::new(-1.866025, 0.866025, 0.0);
        l2.append(Point::new(1.0, 0.0));
        l2.append(Point::new(1.866025, 0.866025));
        f.append(l2);

        let mut l3: TessellationLine = TessellationLine::new(-1.866025, -0.866025, 0.0);
        l3.append(Point::new(1.866025, 0.866025));
        l3.append(Point::new(1.0, 1.73205));
        f.append(l3);

        f
    }
}
