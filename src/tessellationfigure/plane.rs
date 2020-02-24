use crate::tessellationfigure::TessellationFigure;
use crate::tessellationline::Point;

pub struct TessellationPlane {}

impl TessellationPlane {
    pub fn grid(
        &self,
        figure: &TessellationFigure,
        width: f32,
        height: f32,
        dscale: f32,
    ) -> Vec<Vec<Point>> {
        let mut grid = Vec::<Vec<Point>>::new();
        let igx = dscale * figure.gridincx;
        let igy = dscale * figure.gridincy;
        let shx = dscale * figure.shiftx;
        let shy = dscale * figure.shifty;
        let mut minx: f32;
        let mut miny: f32;
        let maxx: f32;
        let mut maxy: f32;

        minx = -igx * 2.0;
        miny = -igy * 2.0;

        maxx = width + igx;
        maxy = height + igy;
        let mut sx;
        let mut sy;
        while miny <= maxy {
            sx = minx;
            sy = miny;
            let mut gridrow = Vec::<Point>::new();
            while sx <= maxx {
                sx += igx;
                gridrow.push(Point::new(sx, sy));
            }
            grid.push(gridrow);
            minx += shx;
            miny += igy;
            if minx > -igx {
                minx -= igx;
                maxy -= shy;
            }
        }
        grid
    }
}
