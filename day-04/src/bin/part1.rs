use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = run(input);
    dbg!(output);
}

fn run(input: &str) -> String {
    let mut sum: i32 = 0;

    for line in input.lines() {
        let mut card_points: i32 = 0;
        let (_, nums) = line.split_once(':').unwrap();
        let (winning, ours) = nums.split_once('|').unwrap();

        let winning_numbers: HashSet<&str> = winning.split_whitespace().collect();
        let our_numbers: HashSet<&str> = ours.split_whitespace().collect();

        let common_count = winning_numbers.intersection(&our_numbers).count();

        if common_count > 0 {
            for i in 1..common_count + 1 {
                dbg!(i);
                if i == 1 {
                    card_points += 1;
                } else {
                    card_points *= 2;
                }
            }
        }

        sum += card_points;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let output = run(input);
        assert_eq!("13".to_string(), output);
    }
}

