pub fn parse(input: &str) -> Vec<Vec<i8>> {
    input
        .as_bytes()
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.iter()
                .map(|&c| (c as char).to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let trees = parse(input);
    let rows = trees.len();
    let cols = trees[0].len();
    let mut vismap = vec![vec![false; cols]; rows];

    for i in 0..rows {
        let mut tallest_tree = -1;
        for j in 0..cols {
            if trees[i][j] > tallest_tree {
                tallest_tree = trees[i][j];
                vismap[i][j] = true;
            }
        }
    }
    for i in 0..rows {
        let mut tallest_tree = -1;
        for j in (0..cols).rev() {
            if trees[i][j] > tallest_tree {
                tallest_tree = trees[i][j];
                vismap[i][j] = true;
            }
        }
    }
    for j in 0..cols {
        let mut tallest_tree = -1;
        for i in 0..rows {
            if trees[i][j] > tallest_tree {
                tallest_tree = trees[i][j];
                vismap[i][j] = true;
            }
        }
    }
    for j in 0..cols {
        let mut tallest_tree = -1;
        for i in (0..rows).rev() {
            if trees[i][j] > tallest_tree {
                tallest_tree = trees[i][j];
                vismap[i][j] = true;
            }
        }
    }

    vismap.iter().flatten().filter(|&&visible| visible).count()
}

pub fn part2(input: &str) -> usize {
    let trees = parse(input);
    let rows = trees.len();
    let cols = trees[0].len();
    let mut scenicmap = vec![vec![0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let mut scenicness = 1;
            let mut count = 0;
            for i2 in (i + 1)..rows {
                count += 1;
                if trees[i2][j] >= trees[i][j] {
                    break;
                }
            }
            scenicness *= count;

            let mut count = 0;
            for i2 in (0..i).rev() {
                count += 1;
                if trees[i2][j] >= trees[i][j] {
                    break;
                }
            }
            scenicness *= count;

            let mut count = 0;
            for j2 in (j + 1)..rows {
                count += 1;
                if trees[i][j2] >= trees[i][j] {
                    break;
                }
            }
            scenicness *= count;

            let mut count = 0;
            for j2 in (0..j).rev() {
                count += 1;
                if trees[i][j2] >= trees[i][j] {
                    break;
                }
            }
            scenicness *= count;

            scenicmap[i][j] = scenicness;
        }
    }

    scenicmap.into_iter().flatten().max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    //#[test]
    //pub fn test_parse() {
    //    let input = include_str!("test-input");
    //    assert_eq!(parse(input), ());
    //}

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 21);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 8);
    }
}
