#[cfg(test)]
mod tests {
    use tessellations::tessellationline::{Point, PointIndexPath, TessellationLine};

    #[test]
    fn test_breakline() {
        let mut l1: TessellationLine = TessellationLine::new(1.0, 0.0, 0.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(0.0, 1.0));

        assert_eq!(
            l1.hitline(Point::new(0.0, 0.5), 0.01),
            Some(PointIndexPath {
                line_index: 0,
                point_index: 0,
                corrp: false
            })
        );
    }

    #[test]
    fn test_breakline_not() {
        let mut l1: TessellationLine = TessellationLine::new(1.0, 0.0, 0.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(0.0, 1.0));

        assert_eq!(l1.hitline(Point::new(0.0, 1.5), 0.01), None);
    }

    #[test]
    fn test_breakline_corrp() {
        let mut l1: TessellationLine = TessellationLine::new(1.0, 0.0, 0.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(0.0, 1.0));

        assert_eq!(
            l1.hitline(Point::new(1.0, 0.5), 0.01),
            Some(PointIndexPath {
                line_index: 0,
                point_index: 0,
                corrp: true
            })
        );
    }

    #[test]
    fn test_breakline_corrp_not() {
        let mut l1: TessellationLine = TessellationLine::new(1.0, 0.0, 0.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(0.0, 1.0));

        assert_eq!(l1.hitline(Point::new(1.1, 0.5), 0.01), None);
    }

    #[test]
    fn test_hitpoint() {
        let mut l1: TessellationLine = TessellationLine::new(1.0, 0.0, 0.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(0.0, 0.5));
        l1.append(Point::new(0.0, 1.0));

        assert_eq!(
            l1.hitpoint(Point::new(0.0, 0.5), 0.01),
            Some(PointIndexPath {
                line_index: 0,
                point_index: 1,
                corrp: false
            })
        );
    }

    #[test]
    fn test_hitpoint_not() {
        let mut l1: TessellationLine = TessellationLine::new(1.0, 0.0, 0.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(0.0, 0.5));
        l1.append(Point::new(0.0, 1.0));

        assert_eq!(l1.hitpoint(Point::new(0.0, 0.4), 0.01), None);
    }
    #[test]
    fn test_hitpoint_corrp() {
        let mut l1: TessellationLine = TessellationLine::new(1.0, 0.0, 0.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(0.0, 0.5));
        l1.append(Point::new(0.0, 1.0));

        assert_eq!(
            l1.hitpoint(Point::new(1.0, 0.5), 0.01),
            Some(PointIndexPath {
                line_index: 0,
                point_index: 1,
                corrp: true
            })
        );
    }

    #[test]
    fn test_hitpoint_not_corrp() {
        let mut l1: TessellationLine = TessellationLine::new(1.0, 0.0, 0.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(0.0, 0.5));
        l1.append(Point::new(0.0, 1.0));

        assert_eq!(l1.hitpoint(Point::new(1.0, 0.4), 0.01), None);
    }

    #[test]
    fn test_line_save_json() {
        let mut l1: TessellationLine = TessellationLine::new(1.0, 0.0, 0.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(0.0, 0.5));
        l1.append(Point::new(0.0, 1.0));
	let j = "{\"points\":[{\"x\":0.0,\"y\":0.0},{\"x\":0.0,\"y\":0.5},{\"x\":0.0,\"y\":1.0}],\"transform\":[1.0,0.0,0.0,1.0,1.0,0.0],\"ci\":[1.0,0.0,0.0,1.0,-1.0,0.0],\"angle\":0.0,\"tx\":1.0,\"ty\":0.0}";
        assert_eq!(
            serde_json::to_string(&l1).expect(""),j);
    }

    #[test]
    fn test_line_load_json() {
        let mut l1: TessellationLine = TessellationLine::new(1.0, 0.0, 0.0);
        l1.append(Point::new(0.0, 0.0));
        l1.append(Point::new(0.0, 0.5));
        l1.append(Point::new(0.0, 1.0));

	let j = "{\"points\":[{\"x\":0.0,\"y\":0.0},{\"x\":0.0,\"y\":0.5},{\"x\":0.0,\"y\":1.0}],\"transform\":[1.0,0.0,0.0,1.0,1.0,0.0],\"ci\":[1.0,0.0,0.0,1.0,-1.0,0.0],\"angle\":0.0,\"tx\":1.0,\"ty\":0.0}";

        let l2 = serde_json::from_str::<TessellationLine>(j).expect("parse error");
        assert_eq!(l1,l2);
    }
}
