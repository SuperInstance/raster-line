//! Bresenham's line algorithm for efficient pixel-level line rasterization.

/// A 2D integer point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Rasterize a line from (x0, y0) to (x1, y1) using Bresenham's algorithm.
/// Returns all pixels that the line passes through, including both endpoints.
pub fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<Point> {
    let mut points = Vec::new();
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx: i32 = if x0 < x1 { 1 } else { -1 };
    let sy: i32 = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        points.push(Point::new(x, y));
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_line() {
        let pts = bresenham_line(0, 0, 5, 0);
        assert_eq!(pts.len(), 6);
        for p in &pts {
            assert_eq!(p.y, 0);
        }
        assert_eq!(pts[0], Point::new(0, 0));
        assert_eq!(pts[5], Point::new(5, 0));
    }

    #[test]
    fn test_vertical_line() {
        let pts = bresenham_line(3, 0, 3, 4);
        assert_eq!(pts.len(), 5);
        for p in &pts {
            assert_eq!(p.x, 3);
        }
    }

    #[test]
    fn test_diagonal_line() {
        let pts = bresenham_line(0, 0, 4, 4);
        assert_eq!(pts.len(), 5);
        for (i, p) in pts.iter().enumerate() {
            assert_eq!(p.x, i as i32);
            assert_eq!(p.y, i as i32);
        }
    }

    #[test]
    fn test_single_point() {
        let pts = bresenham_line(5, 5, 5, 5);
        assert_eq!(pts.len(), 1);
        assert_eq!(pts[0], Point::new(5, 5));
    }

    #[test]
    fn test_negative_slope() {
        let pts = bresenham_line(0, 5, 5, 0);
        assert_eq!(pts[0], Point::new(0, 5));
        assert_eq!(pts.last().unwrap(), &Point::new(5, 0));
    }

    #[test]
    fn test_reverse_direction() {
        let forward = bresenham_line(0, 0, 5, 5);
        let backward = bresenham_line(5, 5, 0, 0);
        assert_eq!(forward.len(), backward.len());
        for (f, b) in forward.iter().zip(backward.iter().rev()) {
            assert_eq!(f, b);
        }
    }

    #[test]
    fn test_steep_line() {
        let pts = bresenham_line(0, 0, 1, 5);
        assert_eq!(pts[0], Point::new(0, 0));
        assert_eq!(*pts.last().unwrap(), Point::new(1, 5));
        // Every pixel should be connected
        for window in pts.windows(2) {
            let dx = (window[1].x - window[0].x).abs();
            let dy = (window[1].y - window[0].y).abs();
            assert!(dx <= 1 && dy <= 1);
        }
    }

    #[test]
    fn test_octant_coverage() {
        // Test all 8 octants - line should always include both endpoints
        let origins = [
            (0, 0, 5, 2),  // shallow +
            (0, 0, 2, 5),  // steep +
            (0, 0, -5, 2), // shallow -
            (0, 0, -2, 5), // steep -
            (0, 0, -5, -2),
            (0, 0, -2, -5),
            (0, 0, 5, -2),
            (0, 0, 2, -5),
        ];
        for (x0, y0, x1, y1) in origins {
            let pts = bresenham_line(x0, y0, x1, y1);
            assert_eq!(pts[0], Point::new(x0, y0));
            assert_eq!(*pts.last().unwrap(), Point::new(x1, y1));
        }
    }
}
