use std::fmt;
use std::fs;

struct PointPair {
    start: [u32; 2],
    end: [u32; 2],
}

impl PointPair {
    fn from_line(s: &str) -> Self {
        // 720,475 -> 720,669
        let parts: Vec<&str> = s.split("->").collect();

        let raw_start: Vec<&str> = parts[0].trim().split(",").collect();
        let raw_end: Vec<&str> = parts[1].trim().split(",").collect();

        let start_x: u32 = raw_start[0].parse().unwrap();
        let start_y: u32 = raw_start[1].parse().unwrap();

        let end_x: u32 = raw_end[0].parse().unwrap();
        let end_y: u32 = raw_end[1].parse().unwrap();

        let start: [u32; 2] = [start_x, start_y];
        let end: [u32; 2] = [end_x, end_y];

        Self { start, end }
    }
}

struct PointPairs {
    points: Vec<PointPair>,
    size: [u32; 2], // [x, y]
    matrix: Vec<Vec<u32>>,
    result: u32,
}

impl fmt::Display for PointPairs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.matrix.iter() {
            write!(f, "{:?}\n", row);
        }
        Ok(())
    }
}

impl PointPairs {
    fn new() -> Self {
        PointPairs {
            points: Vec::new(),
            size: Default::default(),
            matrix: Vec::new(),
            result: 0,
        }
    }

    fn add_point_pair(&mut self, p: PointPair) {
        let x = vec![p.start[0], p.end[0], self.size[0]];
        let y = vec![p.start[1], p.end[1], self.size[1]];
        self.points.push(p);
        self.size[0] = *x.iter().max().unwrap();
        self.size[1] = *y.iter().max().unwrap();
    }

    fn init_matrix(&mut self) {
        let x = self.size[0] as usize;
        let y = self.size[1] as usize;
        self.matrix = vec![vec![0; x + 1]; y + 1];
    }

    fn process(&mut self) -> u32 {
        for point in self.points.iter() {
            // println!("Point: {:?} -> {:?}", point.start, point.end);
            if !is_horizontal_or_vertical(point.start, point.end) {
                continue
            }
            let points_between = get_points_between(point.start, point.end);
            // println!("Points between: {:?}", points_between);
            for (_, point_between) in points_between.iter().enumerate() {
                let x = point_between[0] as usize;
                let y = point_between[1] as usize;
                self.matrix[y][x] += 1;
                if self.matrix[y][x] == 2 {
                    self.result += 1
                }
            }
        }
        self.result
    }
}

fn is_horizontal_or_vertical(a: [u32; 2], b: [u32; 2]) -> bool {
    a[0] == b[0] || a[1] == b[1]
}

fn get_points_between(a: [u32; 2], b: [u32; 2]) -> Vec<[u32; 2]> {
    let [x1, y1] = a;
    let [x2, y2] = b;
    let reorder = |x, y| {
        if x > y {
            return [y, x];
        }
        return [x, y];
    };

    let mut result: Vec<[u32; 2]> = Vec::new();

    if x1 == x2 {
        let [low, high] = reorder(y1, y2);
        for i in low..high + 1 {
            result.push([x1, i]);
        }
        return result;
    } else {
        let [low, high] = reorder(x1, x2);
        for i in low..high + 1 {
            result.push([i, y1]);
        }
        return result;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n");

    let mut point_pairs = PointPairs::new();

    for line in lines {
        if line.len() == 0 {
            break;
        }
        let point_pair = PointPair::from_line(line);
        point_pairs.add_point_pair(point_pair)
    }
    point_pairs.init_matrix();
    let result = point_pairs.process();
    // println!("matrix: \n{}", point_pairs);
    println!("result: {}", result);
}
