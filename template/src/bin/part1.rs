fn main() {
    let input = include_str!("./input1.txt");
    let output = run(input);
    dbg!(output);
}

fn run(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "";
        let output = run(input);
        assert_eq!("".to_string(), output);
    }
}

