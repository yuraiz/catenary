use nannou::prelude::*;

use super::Handle;

const SECTIONS: usize = 50;

#[derive(Debug)]
pub struct Chain {
    length: f32,
    indexes: (usize, usize),
}

impl Chain {
    pub fn new(start: usize, end: usize, length: f32) -> Self {
        Self {
            length,
            indexes: (start, end),
        }
    }

    pub fn draw(&self, handles: &[Handle], draw: &Draw) {
        let (a, b) = self.handle_pair(handles);
        draw_chain(a.pos(), b.pos(), draw, self.length)
    }

    pub fn update_physics(&self, handles: &mut [Handle], speed: f32) {
        let (a, b) = self.indexes;
        if a == b {
            return;
        }

        let (a, b) = self.handle_pair_mut(handles);
        let dist = a.distance(b.pos());

        if dist < self.length {
            return;
        }

        let force = (dist - self.length) * speed * 0.01;

        let target = match (a.is_selected(), b.is_selected()) {
            (true, true) => return,
            (true, false) => a.pos(),
            (false, true) => b.pos(),
            (false, false) => (a.pos() + b.pos()) * 0.5,
        };

        if !a.is_selected() {
            a.set_pos(a.pos() + (target - a.pos()) * force);
        }

        if !b.is_selected() {
            b.set_pos(b.pos() + (target - b.pos()) * force);
        }
    }

    fn handle_pair<'a>(&self, handles: &'a [Handle]) -> (&'a Handle, &'a Handle) {
        let (a, b) = self.indexes;
        (&handles[a], &handles[b])
    }

    fn handle_pair_mut<'a>(&self, handles: &'a mut [Handle]) -> (&'a mut Handle, &'a mut Handle) {
        let (a, b) = self.indexes;
        let a = a.min(b);
        let b = b.max(a);
        let (start, end) = handles.split_at_mut(b);
        (&mut start[a], &mut end[0])
    }
}

// Solve for a using Newton-Raphson iteration (38), (39) and (40) with h = 100 , v = 20 and L = 110

/// h: horizontal distance
/// v: vertical distance
/// l: lenght of the catenary
fn catenary(h: f32, v: f32, l: f32) -> (f32, f32, f32) {
    // https://www.mygeodesy.id.au/documents/Catenary%20Curve.pdf

    // Approximation of a
    let mut a = h / 24.0.sqrt() * (h / ((l * l - v * v).sqrt() - h)).sqrt();

    if a.is_nan() {
        return (a, a, a);
    }

    for _ in 0..50 {
        // Newton-Raphson iteration
        let h_div_2a: f32 = h / (2.0 * a);

        let f = 2.0 * a * h_div_2a.sinh() - (l * l - v * v).sqrt();
        let f2 = 2.0 * (h_div_2a.sinh() - h_div_2a * h_div_2a.cosh());

        a -= f / f2
    }

    let x2 = h * 0.5 + a * (v / l).atanh();
    let x1 = x2 - h;

    (a, x1, x2)
}

#[test]
fn test_catenary() {
    let (a, x1, x2) = catenary(100.0, 20.0, 110.0);

    assert!((a - 72.288087476).abs() < 0.0001);

    assert!((x2 - 63.291060536).abs() < 0.0001);
    assert!((x1 - (-36.708939464)).abs() < 0.0001);
}

fn draw_chain(a: Point2, b: Point2, draw: &Draw, length: f32) {
    let (p1, p2) = if a.x < b.x { (a, b) } else { (b, a) };

    let sign = (p2.y - p1.y).signum();

    let h = (p1.x - p2.x).abs();
    let v = (p1.y - p2.y).abs();
    let l = length;

    let (a, x1, x2) = catenary(h, v, l);

    if !a.is_finite() || a < 0.05 {
        let dist = p1.distance(p2);

        if dist > l {
            // Draw straight line
            draw.line().start(p1).end(p2).color(WHITE);
        } else {
            // Draw hanging chain
            let top = if p1.y > p2.y { p1 } else { p2 };

            draw.line()
                .start(top)
                .end(top - pt2(0.0, (dist + l) * 0.5))
                .color(WHITE);
        }

        return;
    }

    let y1 = a * (x1 / a).cosh();
    let y2 = a * (x2 / a).cosh();

    let x_shift = if sign > 0.0 { -x1 } else { x2 };

    let map_x = |x| sign * x + x_shift + p1.x.min(p2.x);
    let map_y = |y| y - y1.min(y2) + p1.y.min(p2.y);

    draw.polyline()
        .points((0..SECTIONS + 1).map(|i| {
            let i = i as f32 / SECTIONS as f32;
            let x = i * (x2 - x1) + x1;

            let y = a * (x / a).cosh();

            pt2(map_x(x), map_y(y))
        }))
        .color(WHITE);
}
