#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum CellType {
    None,
    Path,
    Color(u8, u8, u8),
    Node,
}

#[derive(Debug)]
pub(crate) struct Configuration {
    nodes: Vec<(f32, f32)>,
    paths: Vec<Path>,
    pub(crate) map: Vec<CellType>,
    pub(crate) size: (u16, u16),
}

impl Configuration {
    pub(crate) fn new(size: (u16, u16)) -> Self {
        Self {
            nodes: vec![],
            paths: vec![],
            map: vec![CellType::None; (size.0 * size.1) as usize],
            size,
        }
    }
    pub fn add_path_cells(&mut self) {
        for (i, cell) in self.map.iter_mut().enumerate() {
            if *cell == CellType::Node {
                continue;
            }
            let cell_coords = (i as f32 % self.size.0 as f32, i as f32 / self.size.0 as f32);
            let mut distances_to_lines = Vec::new();
            if i == 0 {
                dbg!(&self.paths);
            }
            for path in &self.paths {
                for pair in path.nodes.windows(2) {
                    let p1 = *pair.first().unwrap();
                    let p2 = *pair.last().unwrap();
                    let line = find_line_equation(p1, p2);
                    let closest_point =
                        find_closest_point_on_line_segment(p1, p2, line, cell_coords);
                    distances_to_lines.push(dist(closest_point, cell_coords));
                }
            }
            distances_to_lines.sort_by(|one, two| {
                if one > two {
                    std::cmp::Ordering::Greater
                } else if one == two {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Less
                }
            });
            let accumulated = distances_to_lines
                .iter()
                .take(2)
                .copied()
                .reduce(|a, b| smooth_min(a, b, 9.));

            if let Some(value) = accumulated {
                *cell = if value <= 2. {
                    CellType::Path
                } else {
                    let activ = activation_function(value) as u8 * 255;
                    CellType::Color(activ, activ, activ)
                }
            }
        }
    }

    pub fn add_node_with_paths(&mut self, node: (f32, f32)) {
        for og_node in &self.nodes {
            self.paths.push(Path {
                nodes: vec![node, *og_node],
            });
        }
        self.map[node.1.round() as usize * self.size.0 as usize + node.0.round() as usize] =
            CellType::Node;
        self.nodes.push(node);
    }

    pub(crate) fn clean_map(&mut self) {
        for cell in self.map.iter_mut() {
            if *cell == CellType::Node {
                continue;
            }
            *cell = CellType::None;
        }
    }
}

#[derive(Debug)]
struct Path {
    nodes: Vec<(f32, f32)>,
}

/// Returns: (slope, offset)
fn find_line_equation(p1: (f32, f32), p2: (f32, f32)) -> (f32, f32) {
    let slope = (p1.1 - p2.1) / (p1.0 - p2.0);
    (slope, p1.1 - p1.0 * slope)
}

fn find_closest_point_to_line(
    p1: (f32, f32),
    p2: (f32, f32),
    line: (f32, f32),
    point: (f32, f32),
) -> (f32, f32) {
    if p1.0 == p2.0 {
        return (p1.0, point.1);
    }
    if p1.1 == p2.1 {
        return (point.0, p1.1);
    }
    let (slope, offset) = line;
    let perpendicular_slope = -1f32 / slope;
    let perpendicular_offset = point.1 - perpendicular_slope * point.0;
    let intersect_x = (offset - perpendicular_offset) / (perpendicular_slope - slope);
    let intersect_y = slope * intersect_x + offset;
    (intersect_x, intersect_y)
}

fn find_closest_point_on_line_segment(
    p1: (f32, f32),
    p2: (f32, f32),
    line: (f32, f32),
    point: (f32, f32),
) -> (f32, f32) {
    let intersect_point = find_closest_point_to_line(p1, p2, line, point);
    if p1.0 == p2.0 {
        let (uppermost, lowermost) = if p1.1 > p2.1 { (p1, p2) } else { (p2, p1) };
        return if intersect_point.1 > uppermost.1 {
            uppermost
        } else if intersect_point.1 < lowermost.1 {
            lowermost
        } else {
            intersect_point
        };
    }

    let (rightmost, leftmost) = if p1.0 > p2.0 { (p1, p2) } else { (p2, p1) };

    if intersect_point.0 < leftmost.0 {
        leftmost
    } else if intersect_point.0 > rightmost.0 {
        rightmost
    } else {
        intersect_point
    }
}

fn dist(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    ((p1.0 * (5. / 8.) - p2.0 * (5. / 8.)).powf(2f32)
        + (p1.1 * (8. / 5.) - p2.1 * (8. / 5.)).powf(2.))
    .sqrt()
}

fn activation_function(x: f32) -> f32 {
    1. / (1. + (x - 4.).exp())
}

fn smooth_min(a: f32, b: f32, k: f32) -> f32 {
    let h = (0.5 + 0.5 * (a - b) / k).max(0.).min(1.);
    a * (1. - h) + b * h - k * h * (1. - h)
}

fn point_is_between_lines(line1: (f32, f32), line2: (f32, f32), point: (f32, f32)) -> bool {
    // That the lines' slopes have different signs means we should check the x-axis to see if point is between
    if line1.0.signum() == line2.0.signum() {
        // Not the same sign means the lines are on diff sides
        (line1.0 * point.0 + line1.1 - point.1).signum()
            != (line2.0 * point.0 + line2.1 - point.1).signum()
    } else {
        ((point.1 - line1.1) / line1.0 - point.1).signum()
            != ((point.1 - line2.1) / line2.0 - point.1).signum()
    }
}

#[cfg(test)]
mod tests {
    use crate::pathing::{find_closest_point_on_line_segment, find_line_equation};

    #[test]
    fn find_closest_point_to_line_segment_correct() {
        let p2 = (10., 10.);
        let p1 = (0., 0.);
        assert_eq!(
            find_closest_point_on_line_segment(p1, p2, find_line_equation(p1, p2), (5., 8.)),
            (6.5, 6.5)
        );
        let p2 = (10., 5.);
        assert_eq!(
            find_closest_point_on_line_segment((0., 0.), p2, find_line_equation(p1, p2), (50., 8.)),
            (10., 5.)
        );
    }
}
