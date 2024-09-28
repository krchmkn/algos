/// Bresenham's line algorithm

fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        points.push((x, y));

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x += sx;
        }

        if e2 < dx {
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
    fn bresenham() {
        let line = bresenham_line(0, 0, 2, 2);
        assert_eq!(line, [(0, 0), (1, 1), (2, 2)])
    }
}
