use std::fs;

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<u32>>,
}

impl Matrix {
    fn new() -> Matrix {
        Matrix {
            data: Default::default(),
        }
    }
}

struct LineReader {
    data: Vec<char>,
    pos: usize,
}

impl LineReader {
    fn new(s: &str) -> Self {
        LineReader {
            data: s.chars().collect(),
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
            let char = self.data[self.pos];
            if char != ' ' {
                buf.push(char)
            }
            self.pos += 1;
        }
        self.pos += 1;
        let number: u32 = std::str::from_utf8(buf.as_bytes()).
            unwrap().
            parse().
            unwrap();
        Some(number)
    }
}

fn main() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let parts = input_str.split("\n").enumerate();

    let mut numbers: Vec<u32> = Vec::new();
    let mut matrices: Vec<Matrix> = Vec::new();
    let mut matrix: Matrix = Matrix::new();

    for (i, part) in parts {
        // handle the first line
        if i == 0 {
            for number in part.split(",") {
                let n = std::str::from_utf8(number.as_bytes())
                    .unwrap()
                    .parse()
                    .unwrap();
                numbers.push(n);
            }
            continue;
        }
        if part.len() == 0 {
            continue;
        }

        let mut tmp :Vec<u32> = Vec::new();

        for number in LineReader::new(part) {
           tmp.push(number)
        }

        matrix.data.push(tmp);
        println!("matrix: {:?}", matrix);
        if matrix.data.len() == 5 {
            matrices.push(matrix);
            matrix = Matrix::new();
        }
    }
    println!("{:?}", matrices);
}
