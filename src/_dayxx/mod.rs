pub fn parse(input: &str) -> () {
    todo!("impl parse")
}

pub fn part1(input: &str) -> usize {
    let data = parse(input);
    todo!("impl part 1")
}

pub fn part2(input: &str) -> usize {
    let data = parse(input);
    todo!("impl part 2")
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_parse() {
        let input = include_str!("test-input");
        assert_eq!(parse(input), ());
    }

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 42);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 1337);
    }
}
