use std::fs;

fn read_input() -> Vec<i32> {
    let data = fs::read_to_string("input.txt").unwrap();
    let parts = data.split("\n");
    let mut out: Vec<i32> = Vec::new();
    for part in parts {
        let val = std::str::from_utf8(part.as_bytes()).unwrap();
        if let Ok(numb) = val.parse() {
            out.push(numb);
        }
    }
    out
}

fn apply_window(size: usize, input: Vec<i32>) -> Result<Vec<i32>, &'static str> {
    if input.len() < size {
        return Err("invalid vec length")
    }
    let mut result: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    for n in 0..input.len() {
        if n < size {
            sum += input[n] as i32;
            if n == 2 {
                result.push(sum);
            }
            continue
        }
        sum += input[n];
        sum -= input[n-size];
        result.push(sum);
    }
    Ok(result)
}

fn number_of_increases(input: Vec<i32>) -> i32 {
    println!("{:?}", input);
    let mut result = 0;
    for n in 1..input.len() {
        if input[n] > input[n - 1] {
            result = result + 1
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let out = number_of_increases(input);
        assert_eq!(out, 7);
    }

    #[test]
    fn prod1() {
        let input = read_input();
        let out = number_of_increases(input);
        println!("{}", out);
        assert_eq!(out, 1288);
    }

    #[test]
    fn test3() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let modified_input = apply_window(3, input);
        let out = number_of_increases(modified_input.unwrap());
        assert_eq!(6, out);
    }

    #[test]
    fn prod2() {
        let input = read_input();
        let modified_input = apply_window(3, input);
        let out = number_of_increases(modified_input.unwrap());
        println!("{}", out);
        assert_eq!(out, 1311);
    }
}
