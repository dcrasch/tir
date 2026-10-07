#[cfg(test)]
mod tests {
    use tessellations::tessellationfigure::{TessellationFigure, TessellationPlane};
    use tessellations::tessellationline::Point;

    fn all_figures() -> Vec<(&'static str, TessellationFigure)> {
        vec![
            ("square", TessellationFigure::square()),
            ("triangle", TessellationFigure::triangle()),
            ("square90", TessellationFigure::square90()),
            ("diamond", TessellationFigure::diamond()),
            ("brick", TessellationFigure::brick()),
            ("hexagon", TessellationFigure::hexagon()),
        ]
    }

    #[test]
    fn test_grid_square() {
        let f = TessellationFigure::square();
        let p = TessellationPlane {};
        // 1x1 area + radius sqrt(2) padding -> origins -2..=2 in both directions.
        let g = p.grid(&f, 1.0, 1.0);
        assert_eq!(g.len(), 5);
        for (r, row) in g.iter().enumerate() {
            let xs: Vec<f32> = row.iter().map(|p| p.x).collect();
            assert_eq!(xs, vec![-2.0, -1.0, 0.0, 1.0, 2.0]);
            assert!(row.iter().all(|p| p.y == r as f32 - 2.0));
        }
    }

    #[test]
    fn test_grid_contains_origin() {
        for (name, f) in all_figures() {
            let g = TessellationPlane {}.grid(&f, 4.0, 4.0);
            assert!(
                g.iter().flatten().any(|p| *p == Point::new(0.0, 0.0)),
                "{name}: (0, 0) missing, tiles won't line up with the figure"
            );
        }
    }

    #[test]
    fn test_grid_brick_rows_are_shifted() {
        let g = TessellationPlane {}.grid(&TessellationFigure::brick(), 2.0, 2.0);
        let row0 = g.iter().find(|r| r[0].y == 0.0).unwrap();
        let row1 = g.iter().find(|r| r[0].y == 1.0).unwrap();
        assert_eq!(row0[0].x.fract(), 0.0);
        assert_eq!(row1[0].x.fract().abs(), 0.5);
    }

    /// Polygon of the figure without the duplicated shared endpoints.
    fn polygon(f: &TessellationFigure) -> Vec<Point> {
        f.points()
            .windows(2)
            .filter_map(|w| (w[0] != w[1]).then_some(w[0]))
            .collect()
    }

    /// Even-odd rule.
    fn inside(poly: &[Point], p: Point) -> bool {
        let mut c = false;
        let n = poly.len();
        for i in 0..n {
            let (a, b) = (poly[i], poly[(i + n - 1) % n]);
            if (a.y > p.y) != (b.y > p.y) && p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y) + a.x {
                c = !c;
            }
        }
        c
    }

    /// Every sample point of the requested area lies in exactly one tile:
    /// no gaps (missing tiles at the edges) and no overlaps (duplicate points).
    #[test]
    fn test_grid_covers_area_exactly_once() {
        let (w, h) = (12.0_f32, 9.0_f32);
        for (name, f) in all_figures() {
            let poly = polygon(&f);
            let g = TessellationPlane {}.grid(&f, w, h);
            let tiles: Vec<Vec<Point>> = g
                .iter()
                .flatten()
                .flat_map(|o| {
                    let poly = &poly;
                    (1..=f.rotdiv).map(move |rot| {
                        let a = std::f32::consts::TAU * rot as f32 / f.rotdiv as f32;
                        let (s, c) = a.sin_cos();
                        poly.iter()
                            .map(|p| Point::new(p.x * c - p.y * s + o.x, p.x * s + p.y * c + o.y))
                            .collect()
                    })
                })
                .collect();

            // Irregular steps so samples don't land exactly on tile edges.
            let steps = 61;
            for iy in 0..steps {
                for ix in 0..steps {
                    let p = Point::new(
                        -w / 2.0 + w * (ix as f32 + 0.37) / steps as f32,
                        -h / 2.0 + h * (iy as f32 + 0.61) / steps as f32,
                    );
                    let hits = tiles.iter().filter(|t| inside(t, p)).count();
                    assert_eq!(hits, 1, "{name}: point {p:?} covered by {hits} tiles");
                }
            }
        }
    }
}
