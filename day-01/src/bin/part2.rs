fn main() {
    let input = include_str!("./input2.txt");
    let output = run(input);
    dbg!(output);
}

fn run(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let mut has_left = false;
        let mut left: char = '0';
        let mut right: char = '0';
        for (i, c) in line.chars().enumerate() {
            let rest_of_line = &line[i..];
            if rest_of_line.starts_with("one") {
                if !has_left {
                    left = '1';
                    right = '1';
                    has_left = true;
                } else {
                    right = '1';
                }
                continue;
            }
            if rest_of_line.starts_with("two") {
                if !has_left {
                    left = '2';
                    right = '2';
                    has_left = true;
                } else {
                    right = '2';
                }
                continue;
            }
            if rest_of_line.starts_with("three") {
                if !has_left {
                    left = '3';
                    right = '3';
                    has_left = true;
                } else {
                    right = '3';
                }
                continue;
            }
            if rest_of_line.starts_with("four") {
                if !has_left {
                    left = '4';
                    right = '4';
                    has_left = true;
                } else {
                    right = '4';
                }
                continue;
            }
            if rest_of_line.starts_with("five") {
                if !has_left {
                    left = '5';
                    right = '5';
                    has_left = true;
                } else {
                    right = '5';
                }
                continue;
            }
            if rest_of_line.starts_with("six") {
                if !has_left {
                    left = '6';
                    right = '6';
                    has_left = true;
                } else {
                    right = '6';
                }
                continue;
            }
            if rest_of_line.starts_with("seven") {
                if !has_left {
                    left = '7';
                    right = '7';
                    has_left = true;
                } else {
                    right = '7';
                }
                continue;
            }
            if rest_of_line.starts_with("eight") {
                if !has_left {
                    left = '8';
                    right = '8';
                    has_left = true;
                } else {
                    right = '8';
                }
                continue;
            }
            if rest_of_line.starts_with("nine") {
                if !has_left {
                    left = '9';
                    right = '9';
                    has_left = true;
                } else {
                    right = '9';
                }
                continue;
            }
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
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let output = run(input);
        assert_eq!("281".to_string(), output);
    }
}

