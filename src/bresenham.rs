pub fn plot_line(mut x0: isize, mut y0: isize, x1: isize, y1: isize) -> Vec<(isize, isize)> {
    let mut points = Vec::new();
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -((y1 - y0).abs());
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        points.push((x0, y0));
        let e2 = 2 * error;
        if e2 >= dy {
            if x0 == x1 { break; }
            error = error + dy;
            x0 = x0 + sx;
        }
        if e2 <= dx {
            if y0 == y1 { break; }
            error = error + dx;
            y0 = y0 + sy;
        }
    }

    points
}