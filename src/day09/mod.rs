use crate::util::{get_2_mut, HashSet};

#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
pub struct Pos {
    x: i32,
    y: i32,
}

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse(input: &str) -> Vec<(Dir, i32)> {
    input
        .lines()
        .map(|line| {
            (
                match line.as_bytes()[0] {
                    b'U' => Dir::Up,
                    b'D' => Dir::Down,
                    b'L' => Dir::Left,
                    b'R' => Dir::Right,
                    c => panic!("unexpected input: {}", c as char),
                },
                line[2..].parse().unwrap(),
            )
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    simulate_rope::<2>(input)
}

pub fn part2(input: &str) -> usize {
    simulate_rope::<10>(input)
}

/// Simulate dragging a rope around by its head, returning the number of unique spots visited by
/// the tail
fn simulate_rope<const N: usize>(input: &str) -> usize {
    let mut visited = HashSet::default();
    visited.insert(Pos::default());

    let mut segments = [Pos::default(); N];

    for (dir, steps) in parse(input) {
        for _ in 0..steps {
            let head = &mut segments[0];
            match dir {
                Dir::Up => head.y += 1,
                Dir::Down => head.y -= 1,
                Dir::Left => head.x -= 1,
                Dir::Right => head.x += 1,
            };

            for i in 1..segments.len() {
                let [a, b] = get_2_mut(&mut segments, i - 1, i);
                // find in what way `a` has moved relative to `b`, and move `b` accordingly
                match (a.x - b.x, a.y - b.y) {
                    (2.., _dy) => {
                        b.y = a.y;
                        b.x = a.x - 1;
                    }
                    (..=-2, _dy) => {
                        b.y = a.y;
                        b.x = a.x + 1;
                    }
                    (_dx, 2..) => {
                        b.y = a.y - 1;
                        b.x = a.x;
                    }
                    (_dx, ..=-2) => {
                        b.y = a.y + 1;
                        b.x = a.x;
                    }
                    _ => {}
                }
            }

            //let head = *segments.first().unwrap();
            let tail = *segments.last().unwrap();
            //println!("H: {head:?} \t\t T: {tail:?}");
            visited.insert(tail);
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input1");
        assert_eq!(part1(input), 13);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input1");
        assert_eq!(part2(input), 1);

        let input = include_str!("test-input2");
        assert_eq!(part2(input), 36);
    }
}
