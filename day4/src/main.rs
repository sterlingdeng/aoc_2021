use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Matrix {
    map: HashMap<u32, [usize; 2]>, // number -> [r, c]
    r: usize,
    c: usize,
    row_sums: [u32; 5],
    col_sums: [u32; 5],
    board_sum: u32,
    won: bool,
}

impl Matrix {
    fn new() -> Matrix {
        Matrix {
            map: HashMap::new(),
            r: 0,
            c: 0,
            row_sums: Default::default(),
            col_sums: Default::default(),
            board_sum: 0,
            won: false,
        }
    }

    fn push(&mut self, val: u32) {
        if self.c == 5 {
            self.c = 0;
            self.r += 1;
        }
        self.map.insert(val, [self.r, self.c]);
        self.c += 1;
        self.board_sum += val;
    }

    fn process_number_and_check_win(&mut self, val: u32) -> Option<[usize; 2]> {
        match self.map.get(&val) {
            Some(position) => {
                let r = position[0];
                let c = position[1];
                self.row_sums[r] += 1;
                self.col_sums[c] += 1;
                self.board_sum -= val;
                if (self.row_sums[r] == 5) || (self.col_sums[c] == 5) {
                    self.won = true;
                    return Some([r, c])
                }
                return None
            },
            None => return None,
        }
    }

    fn get_board_sum(&mut self) -> u32 {
        self.board_sum
    }
}

struct LineReader {
    data: Vec<char>,
    pos: usize,
}

impl LineReader {
    fn new(input: &str) -> Self {
        LineReader {
            data: input.chars().collect(),
            pos: 0,
        }
    }
}

impl Iterator for LineReader {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            return None;
        }
        let mut buf: String = String::new();
        for _ in 0..2 {
            let c = self.data[self.pos];
            if c != ' ' {
                buf.push(self.data[self.pos]);
            }
            self.pos += 1
        }
        self.pos += 1;
        let number: u32 = std::str::from_utf8(buf.as_bytes())
            .unwrap()
            .parse()
            .unwrap();

        Some(number)
    }
}

fn main() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let lines = input_str.split("\n").enumerate();
    // numbers is the bingo numbers
    let mut numbers: Vec<u32> = Vec::new();
    let mut matrices: Vec<Matrix> = Vec::new();
    let mut matrix: Matrix = Matrix::new();
    let mut j = 0;
    for (i, line) in lines {
        // handle the first line
        if i == 0 {
            for number in line.split(",") {
                let n = std::str::from_utf8(number.as_bytes())
                    .unwrap()
                    .parse()
                    .unwrap();
                numbers.push(n);
            }
            continue;
        }
        if line.len() == 0 {
            continue;
        }

        let reader = LineReader::new(line);
        for number in reader {
            matrix.push(number);
        }
        j += 1;
        if j == 5 {
            matrices.push(matrix);
            j = 0;
            matrix = Matrix::new();
        }
    }

    for bingo_number in numbers.iter_mut() {
        for (i, matrix) in matrices.iter_mut().enumerate() {
            if matrix.won {
                continue
            }
            match matrix.process_number_and_check_win(*bingo_number) {
                Some(_) => {
                    println!("board #{} won with number {} with unmarked number sum of {}", i, bingo_number, matrix.get_board_sum());
                    continue
                }
                None => continue
            }
        }
    }
}
