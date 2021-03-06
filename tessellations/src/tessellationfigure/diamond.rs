use crate::tessellationfigure::TessellationFigure;
use crate::tessellationline::{Point, TessellationLine};
use crate::tessellationshape::TessellationShape;

impl TessellationFigure {
    /// basic figure a hexagon
    pub fn diamond() -> Self {
        let mut f: TessellationFigure = TessellationFigure::new();
        f.gridincx = 1.73205;
        f.gridincy = 1.5;
        f.rotdiv = 3;
        f.is_reversed = true;
        f.shiftx = 0.866_025;
        f.shifty = 0.0;
        f.shape = TessellationShape::S;

        let mut l1: TessellationLine = TessellationLine::new(0.0, 0.0, -240.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(0.0, 1.0));
        f.append(l1);

        let mut l2: TessellationLine = TessellationLine::new(1.73205, 0.0, -120.0);
        l2.append(Point::new(0.0, 1.0));
        l2.append(Point::new(0.866_025, 0.5));
        f.append(l2);

        f
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_polydiamond() {
        let f = TessellationFigure::diamond();
        assert_eq!(
            f.points(),
            vec![
                Point::new(0.0, 0.0),
                Point::new(0.0, 1.0),
                Point::new(0.0, 1.0),
                Point::new(0.866025, 0.5),
                Point::new(0.86602473, 0.49999955),
                Point::new(0.86602455, -0.50000006),
                Point::new(0.86602545, -0.4999999),
                Point::new(0.0, 0.0),
            ]
        );
    }
}
