use crate::tessellationfigure::TessellationFigure;
use crate::tessellationline::Point;

pub struct TessellationPlane {}

impl TessellationPlane {
    /// for a figure and view and scale generate a grid of grid points
    pub fn grid(&self, figure: &TessellationFigure, width: f32, height: f32) -> Vec<Vec<Point>> {
        let mut grid = Vec::<Vec<Point>>::new();
        let igx = figure.gridincx;
        let igy = figure.gridincy;
        let shx = figure.shiftx;
        let shy = figure.shifty;
        let mut minx: f32 = -igx * 2.0 - width / 2.0;
        let mut miny: f32 = -igy * 2.0 - height / 2.0;
        let maxx: f32 = width / 2.0 + igx;
        let mut maxy: f32 = height / 2.0 + igy;
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
