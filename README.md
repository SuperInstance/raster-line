# raster-line

Rasterization primitives for 2D graphics in pure Rust.

## Features

- **Bresenham's line algorithm** — pixel-perfect line rasterization
- **Midpoint circle algorithm** — circle outline and filled circle
- **Scanline fill** — polygon rasterization using scanline algorithm
- **Edge handling** — polygon edge representation with inverse slope
- **Polygon utilities** — area computation, CCW detection

## Usage

```rust
use raster_line::{bresenham_line, midpoint_circle, scanline_fill, rasterize_polygon};

// Bresenham line
let pixels = bresenham_line(0, 0, 10, 5);

// Circle
let circle = midpoint_circle(5, 5, 3);

// Fill a triangle
let vertices = vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)];
let filled = scanline_fill(&vertices);
```

Zero external dependencies. Pure `std` Rust.

## License

MIT
