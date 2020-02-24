#[cfg(test)]
mod tests {
    use tessellation::tessellationfigure::{TessellationFigure, TessellationPlane};
    use tessellation::tessellationline::{Point, PointIndexPath, TessellationLine};

    #[test]
    fn test_grid_square() {
        let f = TessellationFigure::square();
        let p = TessellationPlane {};
        assert_eq!(
            p.grid(&f, 2.0, 2.0, 1.0),
            vec![
                vec![
                    Point::new(-1.0, -2.0),
                    Point::new(0.0, -2.0),
                    Point::new(1.0, -2.0),
                    Point::new(2.0, -2.0),
                    Point::new(3.0, -2.0),
                    Point::new(4.0, -2.0)
                ],
                vec![
                    Point::new(-1.0, -1.0),
                    Point::new(0.0, -1.0),
                    Point::new(1.0, -1.0),
                    Point::new(2.0, -1.0),
                    Point::new(3.0, -1.0),
                    Point::new(4.0, -1.0)
                ],
                vec![
                    Point::new(-1.0, 0.0),
                    Point::new(0.0, 0.0),
                    Point::new(1.0, 0.0),
                    Point::new(2.0, 0.0),
                    Point::new(3.0, 0.0),
                    Point::new(4.0, 0.0)
                ],
                vec![
                    Point::new(-1.0, 1.0),
                    Point::new(0.0, 1.0),
                    Point::new(1.0, 1.0),
                    Point::new(2.0, 1.0),
                    Point::new(3.0, 1.0),
                    Point::new(4.0, 1.0)
                ],
                vec![
                    Point::new(-1.0, 2.0),
                    Point::new(0.0, 2.0),
                    Point::new(1.0, 2.0),
                    Point::new(2.0, 2.0),
                    Point::new(3.0, 2.0),
                    Point::new(4.0, 2.0)
                ],
                vec![
                    Point::new(-1.0, 3.0),
                    Point::new(0.0, 3.0),
                    Point::new(1.0, 3.0),
                    Point::new(2.0, 3.0),
                    Point::new(3.0, 3.0),
                    Point::new(4.0, 3.0)
                ]
            ]
        );
    }

    #[test]
    fn test_grid_triangle() {
        let f = TessellationFigure::triangle();
        let p = TessellationPlane {};
        assert_eq!(
            p.grid(&f, 10.0, 10.0, 1.0),
            vec![
                vec![
                    Point::new(-1.0, -2.0),
                    Point::new(0.0, -2.0),
                    Point::new(1.0, -2.0),
                    Point::new(2.0, -2.0),
                    Point::new(3.0, -2.0),
                    Point::new(4.0, -2.0)
                ],
                vec![
                    Point::new(-1.0, -1.0),
                    Point::new(0.0, -1.0),
                    Point::new(1.0, -1.0),
                    Point::new(2.0, -1.0),
                    Point::new(3.0, -1.0),
                    Point::new(4.0, -1.0)
                ],
                vec![
                    Point::new(-1.0, 0.0),
                    Point::new(0.0, 0.0),
                    Point::new(1.0, 0.0),
                    Point::new(2.0, 0.0),
                    Point::new(3.0, 0.0),
                    Point::new(4.0, 0.0)
                ],
                vec![
                    Point::new(-1.0, 1.0),
                    Point::new(0.0, 1.0),
                    Point::new(1.0, 1.0),
                    Point::new(2.0, 1.0),
                    Point::new(3.0, 1.0),
                    Point::new(4.0, 1.0)
                ],
                vec![
                    Point::new(-1.0, 2.0),
                    Point::new(0.0, 2.0),
                    Point::new(1.0, 2.0),
                    Point::new(2.0, 2.0),
                    Point::new(3.0, 2.0),
                    Point::new(4.0, 2.0)
                ],
                vec![
                    Point::new(-1.0, 3.0),
                    Point::new(0.0, 3.0),
                    Point::new(1.0, 3.0),
                    Point::new(2.0, 3.0),
                    Point::new(3.0, 3.0),
                    Point::new(4.0, 3.0)
                ]
            ]
        );
    }
}