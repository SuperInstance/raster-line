//! Edge representation for polygon rasterization.

/// An edge of a polygon, used in scanline rasterization.
/// Stores the y-range and the inverse slope for interpolation.
#[derive(Debug, Clone)]
pub struct Edge {
    pub y_min: i32,
    pub y_max: i32,
    pub x_at_ymin: f64,
    pub slope_inv: f64,
}

impl Edge {
    /// Create a new edge from two vertices (x0, y0) and (x1, y1).
    /// Automatically orders so y_min <= y_max.
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        let (y_min, y_max, x_at_ymin) = if y0 <= y1 {
            (y0 as i32, y1 as i32, x0)
        } else {
            (y1 as i32, y0 as i32, x1)
        };
        let dy = y1 - y0;
        let slope_inv = if dy.abs() < 1e-10 {
            0.0
        } else {
            (x1 - x0) / dy
        };

        Edge {
            y_min,
            y_max,
            x_at_ymin,
            slope_inv,
        }
    }

    /// Compute the x-coordinate at a given y scanline.
    pub fn x_at(&self, y: i32) -> f64 {
        self.x_at_ymin + self.slope_inv * (y as f64 - self.y_min as f64)
    }

    /// Returns true if this edge is active at the given y scanline.
    pub fn is_active_at(&self, y: i32) -> bool {
        y >= self.y_min && y < self.y_max
    }

    /// Build edges from a list of vertex pairs forming a polygon.
    pub fn from_vertices(vertices: &[(f64, f64)]) -> Vec<Edge> {
        if vertices.len() < 2 {
            return Vec::new();
        }
        let mut edges = Vec::new();
        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            let (x0, y0) = vertices[i];
            let (x1, y1) = vertices[j];
            // Skip horizontal edges
            if (y0 - y1).abs() < 1e-10 {
                continue;
            }
            edges.push(Edge::new(x0, y0, x1, y1));
        }
        edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_basic() {
        let e = Edge::new(0.0, 0.0, 10.0, 10.0);
        assert_eq!(e.y_min, 0);
        assert_eq!(e.y_max, 10);
        assert!((e.x_at(5) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_edge_horizontal_skip() {
        let vertices = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];
        let edges = Edge::from_vertices(&vertices);
        // Horizontal edge (0,0)-(10,0) and (0,10)-(0,0) should be skipped
        // Non-horizontal: (10,0)-(10,10), (10,10)-(0,10), (0,10)-(0,0) is horizontal? No: 10→0 is non-horizontal
        // Actually edges are: (0,0)-(10,0) horiz SKIP, (10,0)-(10,10) keep, (10,10)-(0,10) horiz SKIP, (0,10)-(0,0) keep
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_edge_inverse_slope() {
        let e = Edge::new(0.0, 0.0, 5.0, 10.0);
        assert!((e.slope_inv - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_edge_vertical() {
        let e = Edge::new(5.0, 0.0, 5.0, 10.0);
        assert!((e.slope_inv).abs() < 1e-10);
        assert!((e.x_at(5) - 5.0).abs() < 1e-10);
    }
}
