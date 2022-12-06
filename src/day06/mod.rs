pub fn parse(input: &str) -> &[u8] {
    input.as_bytes()
}

pub fn part1(input: &str) -> usize {
    solve::<4>(input)
}

pub fn part2(input: &str) -> usize {
    solve::<14>(input)
}

/// Find the starting index of the first window of size N, which contains only unique elements.
fn solve<const N: usize>(input: &str) -> usize {
    let mut windows = parse(input).array_windows::<N>().enumerate();

    while let Some((i, window)) = windows.next() {
        // find the index of the first element of the last pair of duplicates that appears in the window
        let first_dup_at = (window.iter().enumerate().rev())
            .find_map(|(j, c)| window[j + 1..].contains(c).then(|| j));

        match first_dup_at {
            // if there is a duplicate, advance the iterator until the duplicate is gone
            Some(j) => {
                windows.advance_by(j).ok();
            }

            // if there wasn't a duplicate, we have our answer
            None => return i + N,
        }
    }

    panic!("No window of unique elements found");
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 7);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 19);
    }
}
