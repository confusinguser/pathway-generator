#[derive(PartialEq, Clone, Copy)]
pub(crate) enum CellType {
    None,
    Path,
    Node,
}

pub(crate) struct Configuration {
    nodes: Vec<(f32, f32)>,
    paths: Vec<Path>,
    pub(crate) map: Vec<CellType>,
    size: (u16, u16),
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
            let cell_coords = (i as f32 / self.size.0 as f32, i as f32 % self.size.0 as f32);
            if *cell == CellType::None {
                for path in &self.paths {
                    let mut distances = Vec::new();
                    if path.nodes.len() == 0 {
                        let closest_point = find_closest_point_to_line_segment(path.endpoints.0, path.endpoints.1, cell_coords);
                        distances.push(dist(closest_point, cell_coords));
                    }
                    for pair in path.nodes.windows(2) {
                        let closest_point = find_closest_point_to_line_segment(*pair.get(0).unwrap(), *pair.get(1).unwrap(), cell_coords);
                        distances.push(dist(closest_point, cell_coords));
                    }
                    let value: f32 = distances.iter().map(|distance| activation_function(*distance)).sum();
                    if value >= 0.8 {
                        *cell = CellType::Path;
                    }
                }
            }
        }
    }

    pub fn add_node_with_paths(&mut self, node: (f32, f32)) {
        for og_node in &self.nodes {
            self.paths.push(Path {
                nodes: vec![node, *og_node],
            });
            self.map[node.1.round() as u16 * self.size.0 + node.0.round() as u16] = CellType::Node;
        }
        self.nodes.push(node);
    }
}

struct Path {
    nodes: Vec<(f32, f32)>,
}

/// Returns: (slope, offset)
fn find_line_equation(p1: (f32, f32), p2: (f32, f32)) -> (f32, f32) {
    let slope = (p1.1 - p2.1) / (p1.0 - p2.0);
    (slope, p1.1 - p1.0 * slope)
}

fn find_closest_point_to_line(p1: (f32, f32), p2: (f32, f32), point: (f32, f32)) -> (f32, f32) {
    let (slope, offset) = find_line_equation(p1, p2);
    let perpendicular_slope = -1f32 / slope;
    let perpendicular_offset = point.1 - perpendicular_slope * point.0;
    let intersect_x = (offset - perpendicular_offset) / (perpendicular_slope - slope);
    let intersect_y = slope * intersect_x + offset;
    (intersect_x, intersect_y)
}

fn find_closest_point_to_line_segment(p1: (f32, f32), p2: (f32, f32), point: (f32, f32)) -> (f32, f32) {
    let intersect_point = find_closest_point_to_line(p1, p2, point);
    let rightmost;
    let leftmost;
    if p1.0 < p2.0 {
        rightmost = p2;
        leftmost = p1;
    } else {
        rightmost = p1;
        leftmost = p2;
    }

    if intersect_point.0 < leftmost.0 {
        leftmost
    } else if intersect_point.0 > rightmost.0 {
        rightmost
    } else {
        intersect_point
    }
}

fn dist(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    ((p1.0 - p2.0).powf(2f32) + (p1.1 - p2.1).powf(2f32)).sqrt()
}

fn activation_function(distance: f32) -> f32 {
    1. / (1. + (distance - 4.).exp())
}