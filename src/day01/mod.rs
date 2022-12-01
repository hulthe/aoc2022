pub fn parse(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|elf_calories| {
            elf_calories
                .lines()
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum()
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let data = parse(input);
    data.into_iter().max().unwrap_or(0)
}

pub fn part2(input: &str) -> usize {
    let mut data = parse(input);
    data.sort();
    data.reverse();
    data[..3].iter().copied().sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 24000);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 45000);
    }
}
