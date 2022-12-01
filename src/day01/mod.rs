use std::collections::BinaryHeap;

pub fn parse(input: &str) -> Vec<u64> {
    let mut data = Vec::with_capacity(100);
    let mut lines = input.lines();

    'outer: loop {
        let mut elf_calories = 0;
        loop {
            match lines.next() {
                None => {
                    data.push(elf_calories);
                    break 'outer;
                }
                Some("") => {
                    data.push(elf_calories);
                    break;
                }
                Some(snack) => elf_calories += snack.parse::<u64>().unwrap(),
            }
        }
    }

    data
}

pub fn part1(input: &str) -> u64 {
    let data = parse(input);
    data.into_iter().max().unwrap_or(0)
}

pub fn part2(input: &str) -> u64 {
    let mut data = BinaryHeap::from(parse(input));
    [data.pop(), data.pop(), data.pop()]
        .into_iter()
        .flatten()
        .sum()
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
