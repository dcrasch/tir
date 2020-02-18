use tessellation::tessellationline::{breakline, distance, hit, Point};

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
fn test_breakline_offright() {
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
fn test_breakline_offleft() {
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
fn test_breakline_offmoreleft() {
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
fn test_breakline_offmoreright() {
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

#[test]
fn test_distance_zeropoint() {
    assert_eq!(
        distance(Point::new(0.0,0.0)),
        0.0
    );
}

#[test]
fn test_distance_3and4() {
    assert_eq!(
        distance(Point::new(3.0,4.0)),
        5.0
    );
}

#[test]
fn test_distance_neg3and4() {
    assert_eq!(
        distance(Point::new(-3.0,4.0)),
        5.0
    );
}

#[test]
fn test_distance_neg3neg4() {
    assert_eq!(
        distance(Point::new(-3.0,-4.0)),
        5.0
    );
}

#[test]
fn test_hitpoint() {
    assert_eq!(
        hit(Point::new(0.0,0.0), Point::new(0.0,0.0),5.0),
        true
    );
}

#[test]
fn test_misspoint() {
    assert_eq!(
        hit(Point::new(0.0,0.0), Point::new(10.0,0.0),5.0),
        false
    );
}