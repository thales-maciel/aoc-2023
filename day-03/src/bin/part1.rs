use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = run(input);
    dbg!(output);
}

#[derive(Debug, PartialEq)]
struct Num {
    value: String,
    line: usize,
    column: usize,
}

struct Scanner {
    input: Vec<char>,
    cursor: usize,
    line: usize,
    column: usize,
}

type Point = (usize, usize);

#[derive(Debug, PartialEq, Clone)]
struct Range {
    start: Point,
    end: Point,
}

#[derive(Debug, PartialEq)]
struct Parsed {
    symbols: HashMap<Point, char>,
    numbers: HashMap<Point, (Range, String)>
}

impl Scanner {
    fn new(input: &str) -> Scanner {
        Self { input: input.chars().collect(), cursor: 0, line: 1, column: 0 }
    }

    fn get_current_char(&self) -> Option<char> {
        self.input.get(self.cursor).copied()
    }

    fn move_next(&mut self) {
        if let Some(c) = self.get_current_char() {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }

        if self.cursor < self.input.len() {
            self.cursor += 1;
        }
    }

    fn parse_number(&mut self) -> Option<Num> {
        let mut value = String::new();
        let start_line = self.line;
        let start_column = self.column;

        while let Some(c) = self.get_current_char() {
            if c.is_digit(10) {
                value.push(c);
                self.move_next();
            } else {
                break;
            }
        }

        if value.is_empty() {
            return None;
        } else {
            return Some(Num { value, line: start_line, column: start_column });
        }
    }

    fn scan(&mut self) -> Parsed {
        let mut numbers: HashMap<Point, (Range, String)> = HashMap::new();
        let mut symbols = HashMap::new();
        while let Some(c) = self.get_current_char() {
            if c.is_digit(10) {
                if let Some(n) = self.parse_number() {
                    let num_str = &n.value.clone();
                    let num_str_len = num_str.len();
                    for i in 0..num_str.len() {
                        let point = (n.line, n.column + i);
                        let range = Range { start: (n.line, n.column), end: (n.line, n.column + num_str_len) };
                        numbers.insert(point, (range, num_str.clone()));
                    }
                    
                };
            } else if c == '\n' {
                self.move_next();
            } else if c == '.' {
                self.move_next();
            } else {
                symbols.insert((self.line, self.column), c);
                self.move_next();
            }
        };
        Parsed { symbols, numbers }
    }
}

fn run(input: &str) -> String {
    let mut sum = 0;
    let mut scanner = Scanner::new(input);
    let parsed = scanner.scan();
    dbg!("{:?}", &parsed.numbers);

    let mut processed_numbers: Vec<Range> = Vec::new();

    for (point, _) in parsed.symbols {
        // check if there is an adjacent number
        let directions = [
            (point.0 - 1, point.1 - 1),
            (point.0 - 1, point.1),
            (point.0 - 1, point.1 + 1),
            (point.0, point.1 - 1),
            (point.0, point.1 + 1),
            (point.0 + 1, point.1 - 1),
            (point.0 + 1, point.1),
            (point.0 + 1, point.1 + 1),
        ];

        for dir in directions.iter() {
            if let Some(n) = parsed.numbers.get(dir) {
                if !processed_numbers.contains(&n.0) {
                    let num: i32 = n.1.parse().unwrap();
                    sum += num;
                    processed_numbers.push(n.0.clone());
                }
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let output = run(input);
        println!("{:?}", input);
        assert_eq!("4361".to_string(), output);
    }
}

