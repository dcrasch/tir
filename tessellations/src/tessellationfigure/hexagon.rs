use crate::tessellationfigure::TessellationFigure;
use crate::tessellationline::{Point, TessellationLine};
use crate::tessellationshape::TessellationShape;

impl TessellationFigure {
    /// basic figure a hexagon
    pub fn hexagon() -> Self {
        let mut f: TessellationFigure = TessellationFigure::new();
        f.gridincx = 3.732_05;
        f.gridincy = 0.866_025;
        f.rotdiv = 1;
        f.is_reversed = false;
        f.shiftx = 1.866_025;
        f.shifty = 0.0;
        f.shape = TessellationShape::S;

        let mut l1: TessellationLine = TessellationLine::new(0.0, 1.732_05, 0.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(1.0, 0.0));
        f.append(l1);

        let mut l2: TessellationLine = TessellationLine::new(-1.866_025, 0.866_025, 0.0);
        l2.append(Point::new(1.0, 0.0));
        l2.append(Point::new(1.866_025, 0.866_025));
        f.append(l2);

        let mut l3: TessellationLine = TessellationLine::new(-1.866_025, -0.866_025, 0.0);
        l3.append(Point::new(1.866_025, 0.866_025));
        l3.append(Point::new(1.0, 1.73205));
        f.append(l3);

        f
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_polyhexagon() {
        let f = TessellationFigure::hexagon();
        assert_eq!(
            f.points(),
            vec![
                Point::new(0.0, 0.0),
                Point::new(1.0, 0.0),
                Point::new(1.0, 0.0),
                Point::new(1.866025, 0.866025),
                Point::new(1.866025, 0.866025),
                Point::new(1.0, 1.73205),
                Point::new(1.0, 1.73205),
                Point::new(0.0, 1.73205),
                Point::new(0.0, 1.73205),
                Point::new(-0.866025, 0.866025),
                Point::new(-0.866025, 0.866025),
                Point::new(0.0, 0.0),
            ]
        );
    }
}
