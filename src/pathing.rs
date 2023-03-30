use std::slice;

#[derive(PartialEq)]
enum CellType {
    None,
    Path,
    Intersect,
    Node,
}

struct Configuration {
    nodes: Vec<(f32, f32)>,
    paths: Paths,
    map: Vec<CellType>,
    size: (u16, u16),
}

struct Path {
    nodes: Vec<(f32, f32)>,
}

impl Path {
    // fn draw_line(&self)
}

struct Paths {
    paths: Vec<Path>,
}

// impl IntoIterator for Paths {
//     type Item = Path;
//     type IntoIter = slice::Iter<Path>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.paths.iter()
//     }
// }

impl Paths {
    fn add_intersects(&self, config: Configuration) {
        for (i, cell) in config.map.iter().enumerate() {
            let cell_coords = (i as f32 / config.size.0 as f32, i as f32 % config.size.0 as f32);
            if *cell == CellType::None {
                for path in config.paths.paths {
                    let mut distances = Vec::new();
                    for pair in path.nodes.windows(2) {
                        let closest_point = find_closest_point_to_line_segment(*pair.get(0).unwrap(), *pair.get(1).unwrap(), cell_coords);
                        distances.push(dist(closest_point, cell_coords));
                    }
                }
            }
        }
    }
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