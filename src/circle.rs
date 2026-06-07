//! Midpoint circle algorithm for efficient circle rasterization.

use crate::bresenham::Point;

/// Rasterize a circle using the midpoint circle algorithm.
/// Returns all 8-way symmetric pixels for the circle centered at (cx, cy) with radius r.
pub fn midpoint_circle(cx: i32, cy: i32, r: i32) -> Vec<Point> {
    let mut points = Vec::new();
    if r < 0 {
        return points;
    }
    if r == 0 {
        points.push(Point::new(cx, cy));
        return points;
    }

    let mut x = 0;
    let mut y = r;
    let mut d = 1 - r;

    while x <= y {
        // Plot all 8 octants
        points.push(Point::new(cx + x, cy + y));
        points.push(Point::new(cx - x, cy + y));
        points.push(Point::new(cx + x, cy - y));
        points.push(Point::new(cx - x, cy - y));
        points.push(Point::new(cx + y, cy + x));
        points.push(Point::new(cx - y, cy + x));
        points.push(Point::new(cx + y, cy - x));
        points.push(Point::new(cx - y, cy - x));

        x += 1;
        if d < 0 {
            d += 2 * x + 1;
        } else {
            y -= 1;
            d += 2 * (x - y) + 1;
        }
    }
    points.sort_by_key(|p| (p.x, p.y));
    points.dedup();
    points
}

/// Returns the filled circle pixels (all pixels within radius r).
pub fn filled_circle(cx: i32, cy: i32, r: i32) -> Vec<Point> {
    let mut points = Vec::new();
    if r < 0 {
        return points;
    }
    for y in -r..=r {
        let x_extent = (((r * r) as i64 - (y * y) as i64).max(0) as f64).sqrt() as i32;
        for x in -x_extent..=x_extent {
            points.push(Point::new(cx + x, cy + y));
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_circle() {
        let pts = midpoint_circle(0, 0, 1);
        assert_eq!(pts.len(), 4);
        assert!(pts.contains(&Point::new(1, 0)));
        assert!(pts.contains(&Point::new(-1, 0)));
        assert!(pts.contains(&Point::new(0, 1)));
        assert!(pts.contains(&Point::new(0, -1)));
    }

    #[test]
    fn test_radius_zero() {
        let pts = midpoint_circle(5, 5, 0);
        assert_eq!(pts.len(), 1);
        assert_eq!(pts[0], Point::new(5, 5));
    }

    #[test]
    fn test_negative_radius() {
        let pts = midpoint_circle(0, 0, -5);
        assert!(pts.is_empty());
    }

    #[test]
    fn test_symmetry() {
        let pts = midpoint_circle(0, 0, 5);
        // Circle should be symmetric in all quadrants
        for p in &pts {
            assert!(
                pts.contains(&Point::new(-p.x, p.y))
                    && pts.contains(&Point::new(p.x, -p.y))
                    && pts.contains(&Point::new(-p.x, -p.y)),
                "Asymmetry at ({}, {})",
                p.x,
                p.y
            );
        }
    }

    #[test]
    fn test_all_pixels_within_radius() {
        let pts = midpoint_circle(0, 0, 3);
        for p in &pts {
            let dist_sq = (p.x * p.x + p.y * p.y) as f64;
            // All pixels should be within ~0.5 of the circle radius
            let r = 3.0;
            assert!(
                dist_sq <= (r + 0.8_f64).powi(2),
                "Point ({}, {}) too far from circle: dist^2={}",
                p.x,
                p.y,
                dist_sq
            );
        }
    }

    #[test]
    fn test_filled_circle_contains_center() {
        let filled = filled_circle(5, 5, 4);
        assert!(filled.contains(&Point::new(5, 5)));
        assert!(filled.contains(&Point::new(5, 6)));
        assert!(filled.contains(&Point::new(6, 5)));
    }

    #[test]
    fn test_filled_circle_area() {
        let pts = filled_circle(0, 0, 2);
        // Area should be roughly π*r² ≈ 12.57
        assert!(pts.len() >= 10 && pts.len() <= 16);
    }

    #[test]
    fn test_circle_offset_center() {
        let pts = midpoint_circle(100, 100, 3);
        // All points should be offset by center
        for p in &pts {
            assert!((p.x - 100).abs() <= 3);
            assert!((p.y - 100).abs() <= 3);
        }
    }
}
