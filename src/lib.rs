//! Rasterization primitives for 2D graphics.
//!
//! Provides Bresenham's line algorithm, midpoint circle rasterization,
//! scanline fill, and polygon rasterization with edge handling.

pub mod bresenham;
pub mod circle;
pub mod edge;
pub mod polygon;
pub mod scanline;

pub use bresenham::bresenham_line;
pub use circle::midpoint_circle;
pub use edge::Edge;
pub use polygon::{rasterize_polygon, Polygon};
pub use scanline::scanline_fill;
