#[cfg(test)]
mod tests {
    use tessellations::tessellationfigure::{TessellationFigure, TessellationPlane};
    use tessellations::tessellationline::Point;

    #[test]
    fn test_grid_square() {
        let f = TessellationFigure::square();
        let p = TessellationPlane {};
        let expected = "[[(-1.5, -2.5), (-0.5, -2.5), (0.5, -2.5), (1.5, -2.5), (2.5, -2.5)], [(-1.5, -1.5), (-0.5, -1.5), (0.5, -1.5), (1.5, -1.5), (2.5, -1.5)], [(-1.5, -0.5), (-0.5, -0.5), (0.5, -0.5), (1.5, -0.5), (2.5, -0.5)], [(-1.5, 0.5), (-0.5, 0.5), (0.5, 0.5), (1.5, 0.5), (2.5, 0.5)], [(-1.5, 1.5), (-0.5, 1.5), (0.5, 1.5), (1.5, 1.5), (2.5, 1.5)]]";
        assert_eq!(format!("{:?}", p.grid(&f, 1.0, 1.0)), expected);
    }
}
