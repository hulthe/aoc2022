use std::ops::RangeInclusive;

type Range = RangeInclusive<usize>;

pub fn parse(input: &str) -> Vec<[Range; 2]> {
    input
        .lines()
        .flat_map(|line| line.split_once(','))
        .map(|(elf1, elf2)| {
            [elf1, elf2]
                .map(|elf| elf.split_once('-').expect("range does not contain '-'"))
                .map(|(from, to)| Range::new(from.parse().unwrap(), to.parse().unwrap()))
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|[elf1, elf2]| contains(elf1, elf2) || contains(elf2, elf1))
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|[elf1, elf2]| overlaps(elf1, elf2))
        .count()
}

/// Check whether a contains b
fn contains(a: &Range, b: &Range) -> bool {
    (a.start() <= b.start()) && (a.end() >= b.end())
}

/// Check whether a overlaps with b
fn overlaps(a: &Range, b: &Range) -> bool {
    (a.end() >= b.start()) && (a.start() <= b.start())
        || (b.end() >= a.start()) && (b.start() <= a.start())
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 2);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 4);
    }
}
