use tessellation::tessellationline::{breakline, Point};

#[test]
fn test_breakline_on() {
    assert_eq!(
        breakline(
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 0.0),
            0.1
        ),
        true
    );
}

#[test]
fn test_breakline_ontop() {
    assert_eq!(
        breakline(
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 0.05),
            0.1
        ),
        true
    );
}

#[test]
fn test_breakline_onright() {
    assert_eq!(
        breakline(
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.01, 0.0),
            0.1
        ),
        false
    );
}

#[test]
fn test_breakline_onleft() {
    assert_eq!(
        breakline(
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(-0.05, 0.0),
            0.1
        ),
        false
    );
}

#[test]
fn test_breakline_offleft() {
    assert_eq!(
        breakline(
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(-0.2, 0.0),
            0.1
        ),
        false
    );
}

#[test]
fn test_breakline_offright() {
    assert_eq!(
        breakline(
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.2, 0.0),
            0.1
        ),
        false
    );
}
