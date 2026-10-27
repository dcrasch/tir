#[cfg(test)]
mod tests {
    use tessellations::tessellationfigure::{TessellationFigure, FIGURES};
    use tessellations::tessellationline::{Point, PointIndexPath};

    fn registry_figures() -> Vec<(&'static str, TessellationFigure)> {
        FIGURES
            .iter()
            .zip(TessellationFigure::all())
            .map(|(info, f)| (info.id, f))
            .collect()
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

    /// How often sample points of the requested area are covered, returning the
    /// number of gaps/overlaps and the first offending point.
    fn coverage(
        name: &str,
        f: &TessellationFigure,
        w: f32,
        h: f32,
        steps: usize,
    ) -> (usize, Option<String>) {
        let poly = polygon(f);
        let placements = f.placements(w, h);
        if placements.len() > 3000 {
            return (
                1,
                Some(format!(
                    "{} placements - tiling does not close up",
                    placements.len()
                )),
            );
        }
        let tiles: Vec<Vec<Point>> = placements
            .iter()
            .map(|pl| {
                poly.iter()
                    .map(|&p| pl.transform.transform_point(p))
                    .collect()
            })
            .collect();

        // Irregular steps so samples don't land exactly on tile edges.
        let mut bad = 0;
        let mut first = None;
        for iy in 0..steps {
            for ix in 0..steps {
                let p = Point::new(
                    -w / 2.0 + w * (ix as f32 + 0.37) / steps as f32,
                    -h / 2.0 + h * (iy as f32 + 0.61) / steps as f32,
                );
                let hits = tiles.iter().filter(|t| inside(t, p)).count();
                if hits != 1 {
                    bad += 1;
                    if first.is_none() {
                        first = Some(format!("{p:?} covered {hits} times"));
                    }
                }
            }
        }
        if bad > 0 && first.is_none() {
            first = Some(format!("{name}: bad"));
        }
        (bad, first)
    }

    /// Every sample point of the requested area lies in exactly one tile:
    /// no gaps (missing tiles) and no overlaps (duplicate tiles).
    #[test]
    fn test_placements_cover_area_exactly_once() {
        let mut failures = Vec::new();
        for (name, f) in registry_figures() {
            let (bad, first) = coverage(name, &f, 8.0, 6.0, 29);
            if bad > 0 {
                failures.push(format!("{name}: {bad} bad samples, first {first:?}"));
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn test_placements_contain_origin() {
        for (name, f) in registry_figures() {
            let poly = polygon(&f);
            let hits = f
                .placements(4.0, 4.0)
                .iter()
                .filter(|pl| {
                    let t: Vec<Point> = poly
                        .iter()
                        .map(|&p| pl.transform.transform_point(p))
                        .collect();
                    inside(&t, Point::new(0.1, 0.1))
                })
                .count();
            assert_eq!(hits, 1, "{name}: origin area not covered exactly once");
        }
    }

    #[test]
    fn test_colour_tables_match_registry() {
        use tessellations::tessellationfigure::color_table;
        for (info, f) in FIGURES.iter().zip(TessellationFigure::all()) {
            assert_eq!(f.ih, Some(info.ih), "{}: isohedral type", info.id);
            let table =
                color_table(info.ih).unwrap_or_else(|| panic!("{}: missing colour table", info.id));
            assert_eq!(table.colors(), info.colors, "{}: table colours", info.id);
            assert_eq!(f.colors, info.colors, "{}: figure colours", info.id);
            assert!(
                (1..=12).contains(&table.aspects),
                "{}: aspect count {}",
                info.id,
                table.aspects
            );
            for p in f.placements(6.0, 6.0) {
                assert!(
                    p.color < f.colors,
                    "{}: colour {} out of range",
                    info.id,
                    p.color
                );
            }
        }
    }

    #[test]
    fn test_isohedral_type_survives_serde() {
        for f in TessellationFigure::all() {
            let json = serde_json::to_string(&f).expect("serialize figure");
            let back: TessellationFigure = serde_json::from_str(&json).expect("parse figure");
            assert_eq!(back.ih, f.ih);
            assert_eq!(back.colors, f.colors);
        }
    }

    #[test]
    fn test_orientation_counts() {
        // Number of tile orientations ("Ways" on tiled.art).
        let expected = [
            ("quad1", 1),
            ("tri2", 2),
            ("quad4", 4),
            ("quad3", 3),
            ("hex1", 1),
        ];
        for (name, ways) in expected {
            let f = TessellationFigure::by_index(
                TessellationFigure::id_index(name).expect("known figure"),
            );
            let mut orientations: Vec<usize> = f
                .placements(6.0, 6.0)
                .iter()
                .map(|p| p.orientation)
                .collect();
            orientations.sort_unstable();
            orientations.dedup();
            assert_eq!(orientations.len(), ways, "{name}: orientations");
        }
    }

    /// Insert a point at the middle of edge 0 and drag it `amount` along the
    /// edge's normal, returning the figure and the new point's path.
    fn edit_edge(f: &mut TessellationFigure, amount: f32) -> PointIndexPath {
        let pts = f.lines[0].points().to_vec();
        let mid = Point::new((pts[0].x + pts[1].x) / 2.0, (pts[0].y + pts[1].y) / 2.0);
        let (dx, dy) = (pts[1].x - pts[0].x, pts[1].y - pts[0].y);
        let len = (dx * dx + dy * dy).sqrt();
        let hit = f.hitline(mid, 0.01).expect("edge 0 is hittable");
        f.insert(hit, mid);
        let path = PointIndexPath {
            line_index: hit.line_index,
            point_index: hit.point_index + 1,
            corrp: false,
        };
        f.update(
            path,
            Point::new(mid.x - dy / len * amount, mid.y + dx / len * amount),
        );
        path
    }

    fn point_count(f: &TessellationFigure) -> usize {
        f.lines.iter().map(|l| l.points().len()).sum()
    }

    #[test]
    fn test_edit_keeps_tiling_valid() {
        let mut failures = Vec::new();
        for (name, mut f) in registry_figures() {
            edit_edge(&mut f, 0.08);
            let (bad, first) = coverage(name, &f, 8.0, 6.0, 21);
            if bad > 0 {
                failures.push(format!("{name}: {bad} bad samples, first {first:?}"));
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn test_dragging_does_not_add_points() {
        for (name, mut f) in registry_figures() {
            let path = edit_edge(&mut f, 0.02);
            // One move in a generic direction: a point sitting on an inner
            // mirror axis splits off its image the first time it leaves the
            // axis, and only then.
            let mid = f.lines[path.line_index].points()[path.point_index];
            let settled = Point::new(mid.x + 0.01, mid.y + 0.01);
            f.update(path, settled);
            let count = point_count(&f);
            for step in 1..=20 {
                let t = 0.005 * step as f32;
                f.update(path, Point::new(settled.x + t, settled.y + t));
            }
            assert_eq!(point_count(&f), count, "{name}: drag added points");
        }
    }

    #[test]
    fn test_editing_an_edge_moves_no_vertex() {
        let mut f = TessellationFigure::square();
        let vertices: Vec<Point> = f.lines.iter().map(|l| l.points()[0]).collect();
        edit_edge(&mut f, 0.1);
        let after: Vec<Point> = f.lines.iter().map(|l| l.points()[0]).collect();
        assert_eq!(after, vertices, "base vertices moved");
        // The pitched edge was copied to the opposite edge.
        assert_eq!(f.lines[0].points().len(), f.lines[2].points().len());
    }
}
