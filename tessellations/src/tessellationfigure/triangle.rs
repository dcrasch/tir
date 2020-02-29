use crate::tessellationfigure::TessellationFigure;
use crate::tessellationline::{Point, TessellationLine};
use crate::tessellationshape::TessellationShape;

impl TessellationFigure {
    pub fn triangle() -> Self {
        let mut f: TessellationFigure = TessellationFigure::new();
        f.gridincx = 3.0;
        f.gridincy = 0.866_025;
        f.rotdiv = 6;
        f.is_reversed = true;
        f.shiftx = 1.5;
        f.shifty = 0.0;
        f.shape = TessellationShape::S;

        let mut l1: TessellationLine = TessellationLine::new(0.0, 0.0, 60.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(-0.5, 0.866_025));
        f.append(l1);

        let mut l2: TessellationLine = TessellationLine::new(0.0, 1.732_05, 180.0);
        l2.append(Point::new(-0.5, 0.866_025));
        l2.append(Point::new(0.0, 0.866_025));
        f.append(l2);

        f
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_polytriangle() {
        let f = TessellationFigure::triangle();
        assert_eq!(
            f.points(),
            vec![
                Point::new(0.0, 0.0),
                Point::new(-0.5, 0.866_025),
                Point::new(-0.5, 0.866_025),
                Point::new(0.0, 0.866_025),
                Point::new(-0.00000007571031, 0.866_025),
                Point::new(0.4999999, 0.8660249),
                Point::new(0.49999964, 0.8660252),
                Point::new(0.0, 0.0),
            ]
        );
    }
}
