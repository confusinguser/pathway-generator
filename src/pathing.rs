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
    pub(crate) paths: Vec<Path>,
    pub(crate) map: CellTypeMap,
    pub(crate) size: (u16, u16),
}

#[derive(Debug, Clone)]
struct CellTypeMap {
    pub(crate) map: Vec<CellType>,
}

impl CellTypeMap {

    pub fn render_path_cells(&mut self, paths: &[Path], size: (u16, u16)) {
        for (i, cell) in self.map.iter_mut().enumerate() {
            if *cell == CellType::Node {
                continue;
            }
            let cell_coords = (i % size.0 as usize, i / size.0 as usize);
            let mut distances_to_lines = Vec::new();
            if i == 0 {
                dbg!(paths);
            }
            for path in paths {
                if path.nodes.first().map_or(false, |x| x.0.round() as usize == cell_coords.0 && x.1.round() as usize == cell_coords.1) {
*cell = CellType::Node;
continue;
                }
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
                } else if (one - two).abs() < f32::EPSILON {
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
                    #[allow(clippy::cast_sign_loss)]
                    let activ = (activation_function(value) * 255.) as u8;
                    CellType::Color(activ, activ, activ)
                }
            }
        }
    }

    fn new(vec: Vec<CellType>) -> CellTypeMap {
        Self {
            map: vec
        }
    }
}

impl Configuration {
    pub(crate) fn new(size: (u16, u16)) -> Self {
        Self {
            nodes: vec![],
            paths: vec![],
            map: CellTypeMap::new(vec![CellType::None; (size.0 * size.1) as usize]),
            size,
        }
    }

    pub fn add_node_with_paths(&mut self, node: (f32, f32)) {
        for og_node in &self.nodes {
            self.paths.push(Path {
                nodes: vec![node, *og_node],
            });
        }
        self.nodes.push(node);
    }

    pub(crate) fn clean_map(&mut self) {
        for cell in &mut self.map {
            if *cell == CellType::Node {
                continue;
            }
            *cell = CellType::None;
        }
    }
    #[must_use]
    fn evaluate_fitness(paths: Vec<Path>, total_num_of_pixels: usize) -> f32 {
        let total_path_length_between_points: f32 = paths
            .iter()
            .map(|path| {
                path.nodes
                    .windows(2)
                    .map(|a| dist(*a.first().unwrap(), *a.last().unwrap()))
                    .sum::<f32>()
            })
            .sum();
        1. / (total_num_of_pixels as f32 * total_path_length_between_points)
    }

    fn get_total_num_of_pixels(map: Vec<CellType>) {
                let total_num_of_pixels: f32 = map // represents total length of path
            .iter()
            .copied()
            .filter(|cell| *cell == CellType::Path)
            .count() as f32;

    }

    fn optimise(&mut self) {
        let mut paths_clone = self.paths.clone();
        let mut map_clone = self.map.clone();
        for path in &mut paths_clone {
            let num_nodes = path.nodes.len();
            for node in path.nodes.iter_mut().skip(1).take(num_nodes - 2) {
                let fitness = Configuration::evaluate_fitness(paths_clone, );
                let original_node = *node;
                let mut new_fitnesses = Vec::new();
                for direction in Direction::directions() {
                    let mut modified_node = original_node;
                    modified_node.0 += direction.get_vector().0 as f32;
                    modified_node.1 += direction.get_vector().1 as f32;
                    *node = modified_node;
                    if 
                }
            }
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_vector(&self) -> (i32, i32) {
        match *self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn directions() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}

#[derive(Debug, Clone)]
struct Path {
    nodes: Vec<(f32, f32)>,
}

/// Returns: (slope, offset)
#[must_use]
fn find_line_equation(p1: (f32, f32), p2: (f32, f32)) -> (f32, f32) {
    let slope = (p1.1 - p2.1) / (p1.0 - p2.0);
    (slope, p1.1 - p1.0 * slope)
}

#[must_use]
fn find_closest_point_to_line(
    p1: (f32, f32),
    p2: (f32, f32),
    line: (f32, f32),
    point: (f32, f32),
) -> (f32, f32) {
    if (p1.0 - p2.0).abs() < f32::EPSILON {
        // Basically p1.0 == p2.0
        return (p1.0, point.1);
    }
    if (p1.1 - p2.1).abs() < f32::EPSILON {
        return (point.0, p1.1);
    }
    let (slope, offset) = line;
    let perpendicular_slope = -1. / slope;
    let perpendicular_offset = point.1 - perpendicular_slope * point.0;
    let intersect_x = (offset - perpendicular_offset) / (perpendicular_slope - slope);
    let intersect_y = slope * intersect_x + offset;
    (intersect_x, intersect_y)
}

#[must_use]
fn find_closest_point_on_line_segment(
    p1: (f32, f32),
    p2: (f32, f32),
    line: (f32, f32),
    point: (f32, f32),
) -> (f32, f32) {
    let intersect_point = find_closest_point_to_line(p1, p2, line, point);
    if (p1.0 - p2.0).abs() < f32::EPSILON {
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

#[must_use]
fn dist(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    ((p1.0 * (5. / 8.) - p2.0 * (5. / 8.)).powf(2.)
        + (p1.1 * (8. / 5.) - p2.1 * (8. / 5.)).powf(2.))
    .sqrt()
}

#[must_use]
fn activation_function(x: f32) -> f32 {
    1. / (1. + (x - 4.).exp())
}

#[must_use]
fn smooth_min(a: f32, b: f32, k: f32) -> f32 {
    let h = (0.5 + 0.5 * (a - b) / k).max(0.).min(1.);
    a * (1. - h) + b * h - k * h * (1. - h)
}
#[must_use]
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
