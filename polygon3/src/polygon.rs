use crate::framebuffer::Framebuffer;
use crate::line::Line;

pub trait Polygon {
    fn polygon(&mut self, points: &[(usize, usize)]);
    fn fill_polygon(&mut self, points: &[(usize, usize)]);
}

impl Polygon for Framebuffer {
    fn polygon(&mut self, points: &[(usize, usize)]) {
        if points.len() < 3 {
            return;
        }

        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];

            self.line(x0, y0, x1, y1);
        }
    }

    fn fill_polygon(&mut self, points: &[(usize, usize)]) {
        if points.len() < 3 {
            return;
        }

        let mut min_y = points[0].1;
        let mut max_y = points[0].1;

        for &(_, y) in points.iter() {
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }

        for y in min_y..=max_y {
            let mut nodes = vec![];
            let mut j = points.len() - 1;
            for i in 0..points.len() {
                let (xi, yi) = points[i];
                let (xj, yj) = points[j];
                if (yi < y && yj >= y) || (yj < y && yi >= y) {
                    let x = xi as f32 + (y as f32 - yi as f32) / (yj as f32 - yi as f32) * (xj as f32 - xi as f32);
                    nodes.push(x as usize);
                }
                j = i;
            }

            nodes.sort();

            for i in (0..nodes.len()).step_by(2) {
                if i + 1 < nodes.len() {
                    for x in nodes[i]..=nodes[i + 1] {
                        self.point(x, y);
                    }
                }
            }
        }
    }
}