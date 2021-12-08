use std::fs;

#[derive(Debug)]
struct Position {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Position {
    fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn update(&mut self, direction: Direction, scalar: u32) {
        match direction {
            Direction::Forward => {
                self.horizontal += scalar;
                self.depth += self.aim * scalar;
            },
            Direction::Down => {
                self.aim += scalar;
            },
            Direction::Up => {
                self.aim -= scalar;
            },
        }
    }
}

enum Direction {
    Forward,
    Down,
    Up,
}

fn parse_direction(s: &str) -> Direction {
    match s {
        "forward" => return Direction::Forward,
        "down" => return Direction::Down,
        "up" => return Direction::Up,
        _ => unreachable!(),
    }
}

fn parse_data_and_calculate(input: String) -> Result<Position, &'static str> {
    let parts = input.split("\n");
    let mut position = Position::new();

    for part in parts {
        let instruction: Vec<&str> = part.split(" ").collect();
        if instruction.len() != 2 {
            break;
        }
        let direction = parse_direction(instruction[0]);
        let scalar: u32 = std::str::from_utf8(instruction[1].as_bytes())
            .unwrap()
            .parse()
            .unwrap();
        position.update(direction, scalar);
    }

    Ok(position)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        let result = parse_data_and_calculate(input).unwrap();
        assert_eq!(result.depth, 10);
        assert_eq!(result.horizontal, 15);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("input.txt").unwrap();
        let result = parse_data_and_calculate(input).unwrap();
        assert_eq!(result.depth, 987457);
        assert_eq!(result.horizontal, 2162);
    }
}
