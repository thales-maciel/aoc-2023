fn main() {
    let input = include_str!("./input1.txt");
    let output = run(input);
    dbg!(output);
}

const RED: usize = 12;
const GREEN: usize = 13;
const BLUE: usize = 14;

fn run(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let parts = line.split(":").collect::<Vec<&str>>();
        let mut exceeded = false;
        let id = parts[0].split_whitespace().collect::<Vec<&str>>()[1];
        let sets = parts[1].split(";");
        for set in sets {
            let pulls = set.split(",");
            for pull in pulls {
                if !exceeded {
                    let parts = pull.split_whitespace().collect::<Vec<&str>>();
                    let count = parts[0].parse::<usize>().unwrap();
                    exceeded = match parts[1] {
                        "red" => count > RED,
                        "green" => count > GREEN,
                        "blue" => count > BLUE,
                        _ => unreachable!()
                    };
                }
            }
        }
        if !exceeded {
            sum += id.parse::<usize>().unwrap();
        }
    };

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let output = run(input);
        assert_eq!("8".to_string(), output);
    }
}

