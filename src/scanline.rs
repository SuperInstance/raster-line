//! Scanline fill algorithm for rasterizing filled polygons.

use crate::edge::Edge;

/// Fill a polygon defined by vertices using scanline rasterization.
/// Returns all (x, y) pixel coordinates inside the polygon.
pub fn scanline_fill(vertices: &[(f64, f64)]) -> Vec<(i32, i32)> {
    if vertices.len() < 3 {
        return Vec::new();
    }

    let edges = Edge::from_vertices(vertices);
    if edges.is_empty() {
        return Vec::new();
    }

    let y_min = edges.iter().map(|e| e.y_min).min().unwrap();
    let y_max = edges.iter().map(|e| e.y_max).max().unwrap();

    let mut pixels = Vec::new();

    for y in y_min..y_max {
        // Find all edge intersections at this scanline
        let mut intersections: Vec<f64> = Vec::new();
        for edge in &edges {
            if edge.is_active_at(y) {
                intersections.push(edge.x_at(y));
            }
        }
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Fill between pairs of intersections
        let mut i = 0;
        while i + 1 < intersections.len() {
            let x_start = intersections[i].ceil() as i32;
            let x_end = intersections[i + 1].floor() as i32;
            for x in x_start..=x_end {
                pixels.push((x, y));
            }
            i += 2;
        }
    }

    pixels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_triangle() {
        let tri = vec![(0.0, 0.0), (4.0, 0.0), (2.0, 4.0)];
        let pixels = scanline_fill(&tri);
        // Should contain some pixels
        assert!(!pixels.is_empty());
        // Should contain the centroid area
        assert!(pixels.contains(&(2, 1)));
    }

    #[test]
    fn test_fill_rectangle() {
        let rect = vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)];
        let pixels = scanline_fill(&rect);
        assert!(!pixels.is_empty());
        // Verify all pixels are roughly within the rectangle bounds
        for &(x, y) in &pixels {
            assert!(x >= -1 && x <= 5, "x={} out of range", x);
            assert!(y >= -1 && y <= 5, "y={} out of range", y);
        }
        // Center should be filled
        assert!(pixels.contains(&(2, 2)));
    }

    #[test]
    fn test_fill_empty_for_line() {
        let line = vec![(0.0, 0.0), (5.0, 5.0)];
        let pixels = scanline_fill(&line);
        assert!(pixels.is_empty());
    }

    #[test]
    fn test_fill_single_pixel_triangle() {
        let tri = vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
        let pixels = scanline_fill(&tri);
        assert!(!pixels.is_empty());
        assert!(pixels.contains(&(0, 0)));
    }

    #[test]
    fn test_fill_no_duplicate_pixels() {
        let tri = vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)];
        let pixels = scanline_fill(&tri);
        let mut sorted = pixels.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), pixels.len(), "Duplicate pixels found");
    }

    #[test]
    fn test_fill_concave() {
        // Concave (L-shaped) polygon
        let concave = vec![
            (0.0, 0.0),
            (6.0, 0.0),
            (6.0, 2.0),
            (3.0, 2.0),
            (3.0, 6.0),
            (0.0, 6.0),
        ];
        let pixels = scanline_fill(&concave);
        assert!(!pixels.is_empty());
        // Top-right area should be empty
        assert!(!pixels.contains(&(4, 4)));
        // Bottom-left area should be filled
        assert!(pixels.contains(&(1, 1)));
    }
}
