use crate::util::HashSet;

use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pos {
    x: i32,
    y: i32,
}

pub fn parse(input: &str) -> HashSet<Pos> {
    let mut rocks = HashSet::default();
    for line in input.lines() {
        for &[from, to] in line
            .split(" -> ")
            .map(|coord| coord.split_once(',').unwrap())
            .map(|(x, y)| Pos {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            })
            .collect::<Vec<_>>()
            .array_windows()
        {
            if from.x != to.x {
                (min(to.x, from.x)..=max(to.x, from.x))
                    .map(|x| Pos { x, ..to })
                    .for_each(|pos| {
                        rocks.insert(pos);
                    });
            } else {
                (min(to.y, from.y)..=max(to.y, from.y))
                    .map(|y| Pos { y, ..to })
                    .for_each(|pos| {
                        rocks.insert(pos);
                    });
            }
        }
    }
    rocks
}

pub fn part1(input: &str) -> usize {
    let mut rocks = parse(input);
    let mut sand_count = 0;
    let abyss = rocks.iter().map(|rock| rock.y).max().unwrap_or(0) + 1;
    'outer: loop {
        let mut sand = Pos { x: 500, y: 0 };
        'inner: loop {
            let potential_falls = [
                Pos {
                    x: sand.x,
                    y: sand.y + 1,
                },
                Pos {
                    x: sand.x - 1,
                    y: sand.y + 1,
                },
                Pos {
                    x: sand.x + 1,
                    y: sand.y + 1,
                },
            ];

            for pos in potential_falls {
                if !rocks.contains(&pos) {
                    sand = pos;
                    if sand.y >= abyss {
                        break 'outer;
                    }
                    continue 'inner;
                }
            }
            sand_count += 1;
            rocks.insert(sand);
            break;
        }
    }

    sand_count
}

pub fn part2(input: &str) -> usize {
    let mut rocks = parse(input);
    let mut sand_count = 0;
    let floor = rocks.iter().map(|rock| rock.y).max().unwrap_or(0) + 1;
    'outer: loop {
        let mut sand = Pos { x: 500, y: 0 };
        'inner: loop {
            let potential_falls = [
                Pos {
                    x: sand.x,
                    y: sand.y + 1,
                },
                Pos {
                    x: sand.x - 1,
                    y: sand.y + 1,
                },
                Pos {
                    x: sand.x + 1,
                    y: sand.y + 1,
                },
            ];

            for pos in potential_falls {
                if !rocks.contains(&pos) {
                    sand = pos;
                    if sand.y >= floor {
                        sand_count += 1;
                        rocks.insert(sand);
                        break 'inner;
                    }
                    continue 'inner;
                }
            }
            sand_count += 1;
            rocks.insert(sand);
            if sand == (Pos { x: 500, y: 0 }) {
                break 'outer;
            }
            break;
        }
    }

    //for y in 0..floor {
    //    for x in 400..=600 {
    //        if rocks.contains(&Pos { x, y }) {
    //            print!("#");
    //        } else {
    //            print!(".");
    //        }
    //    }
    //    println!();
    //}

    sand_count
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 24);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 93);
    }
}
