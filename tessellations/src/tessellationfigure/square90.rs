use crate::tessellationfigure::TessellationFigure;
use crate::tessellationline::{Point, TessellationLine};
use crate::tessellationshape::TessellationShape;

impl TessellationFigure {
    /// Basic figure a square rotatated 90 degrees
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_polysquare90() {
        let f = TessellationFigure::square90();
        assert_eq!(
            f.points(),
            vec![
                Point::new(0.0, 0.0),
                Point::new(0.0, 1.0),
                Point::new(0.0, 1.0),
                Point::new(1.0, 1.0),
                Point::new(1.0, 0.99999994),
                Point::new(1.0, -0.00000004371139),
                Point::new(1.0, 0.000000011924881),
                Point::new(0.0, 0.0),
            ]
        );
    }
}
