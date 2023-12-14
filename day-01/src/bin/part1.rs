fn main() {
    let input = include_str!("./input1.txt");
    let output = run(input);
    dbg!(output);
}

fn run(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let mut has_left = false;
        let mut left: char = '0';
        let mut right: char = '0';
        for c in line.chars() {
            if c.is_numeric() {
                if !has_left {
                    left = c;
                    right = c;
                    has_left = true;
                } else {
                    right = c;
                }
                continue;
            }
        }
        let num: i32 = format!("{}{}", left, right).parse().unwrap();
        sum += num;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        ";
        let output = run(input);
        assert_eq!("142".to_string(), output);
    }
}

