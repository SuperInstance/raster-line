//! Polygon rasterization combining edge table and scanline algorithms.

use crate::bresenham::Point;
use crate::scanline::scanline_fill;

/// A simple polygon defined by its vertices.
#[derive(Debug, Clone)]
pub struct Polygon {
    pub vertices: Vec<(f64, f64)>,
}

impl Polygon {
    pub fn new(vertices: Vec<(f64, f64)>) -> Self {
        Self { vertices }
    }

    /// Returns true if the polygon has enough vertices (>= 3).
    pub fn is_valid(&self) -> bool {
        self.vertices.len() >= 3
    }

    /// Compute the signed area of the polygon.
    /// Positive = counter-clockwise, Negative = clockwise.
    pub fn signed_area(&self) -> f64 {
        let n = self.vertices.len();
        if n < 3 {
            return 0.0;
        }
        let mut area = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            let (xi, yi) = self.vertices[i];
            let (xj, yj) = self.vertices[j];
            area += xi * yj - xj * yi;
        }
        area / 2.0
    }

    /// Check if the polygon is counter-clockwise.
    pub fn is_ccw(&self) -> bool {
        self.signed_area() > 0.0
    }
}

/// Rasterize a filled polygon and return all interior pixels.
pub fn rasterize_polygon(vertices: &[(f64, f64)]) -> Vec<Point> {
    scanline_fill(vertices)
        .into_iter()
        .map(|(x, y)| Point::new(x, y))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_validity() {
        let p = Polygon::new(vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]);
        assert!(p.is_valid());
        let invalid = Polygon::new(vec![(0.0, 0.0), (1.0, 0.0)]);
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_signed_area_triangle() {
        let tri = Polygon::new(vec![(0.0, 0.0), (4.0, 0.0), (0.0, 4.0)]);
        let area = tri.signed_area();
        assert!((area - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_signed_area_square() {
        let sq = Polygon::new(vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]);
        assert!((sq.signed_area() - 16.0).abs() < 1e-10);
    }

    #[test]
    fn test_ccw_detection() {
        let ccw = Polygon::new(vec![(0.0, 0.0), (4.0, 0.0), (2.0, 4.0)]);
        assert!(ccw.is_ccw());
        let cw = Polygon::new(vec![(0.0, 0.0), (2.0, 4.0), (4.0, 0.0)]);
        assert!(!cw.is_ccw());
    }

    #[test]
    fn test_rasterize_basic() {
        let tri = vec![(0.0, 0.0), (6.0, 0.0), (3.0, 6.0)];
        let pixels = rasterize_polygon(&tri);
        assert!(!pixels.is_empty());
        assert!(pixels.iter().any(|p| p.x == 3 && p.y >= 0 && p.y <= 5));
    }
}
