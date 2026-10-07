use crate::tessellationfigure::TessellationFigure;
use crate::tessellationline::Point;

pub struct TessellationPlane {}

impl TessellationPlane {
    /// Generate the grid points (tile origins) needed to cover a `width` x `height`
    /// area centered on the figure origin `(0, 0)`.
    ///
    /// The tile origins form a lattice spanned by the two figure vectors
    ///
    /// ```text
    /// a = (gridincx, 0)          next tile in the same row
    /// b = (shiftx,   gridincy)   next row
    /// ```
    ///
    /// so the point for row `r`, column `k` is `r * b + k * a`. Row 0, column 0 is
    /// `(0, 0)`, which means the tile drawn there lines up with the figure itself.
    /// Draw the figure `rotdiv` times at each point, rotated by `360° / rotdiv`.
    ///
    /// Every tile that overlaps the area is included: the area is padded by
    /// the figure's radius (its farthest point from the origin), so tiles are
    /// never missing at the edges, for any grid size.
    ///
    /// Rows are ordered by increasing y and points in a row by increasing x.
    pub fn grid(&self, figure: &TessellationFigure, width: f32, height: f32) -> Vec<Vec<Point>> {
        let igx = figure.gridincx;
        let igy = figure.gridincy;
        let shx = figure.shiftx;
        if igx <= 0.0 || igy <= 0.0 {
            return Vec::new();
        }

        // Any tile whose origin is farther than this from the area can't overlap it.
        let radius = figure
            .points()
            .iter()
            .map(|p| p.to_vector().length())
            .fold(0.0, f32::max);

        let xmin = -width / 2.0 - radius;
        let xmax = width / 2.0 + radius;
        let ymin = -height / 2.0 - radius;
        let ymax = height / 2.0 + radius;

        // Integer row/column indices, multiplied out (no accumulated float error).
        let r0 = (ymin / igy).floor() as i32;
        let r1 = (ymax / igy).ceil() as i32;
        (r0..=r1)
            .map(|r| {
                let rowx = r as f32 * shx;
                let y = r as f32 * igy;
                let k0 = ((xmin - rowx) / igx).floor() as i32;
                let k1 = ((xmax - rowx) / igx).ceil() as i32;
                (k0..=k1)
                    .map(|k| Point::new(rowx + k as f32 * igx, y))
                    .collect()
            })
            .collect()
    }
}
