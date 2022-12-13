use std::collections::BTreeSet;

use crate::util::HashMap;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    x: i32,
    y: i32,
}

pub struct Map {
    heights: HashMap<Pos, u8>,
    start: Pos,
    finish: Pos,
}

pub fn parse(input: &str) -> Map {
    let mut start = None;
    let mut finish = None;
    let mut heights = HashMap::default();

    for (y, line) in input.as_bytes().split(|&c| c == b'\n').enumerate() {
        for (x, &height) in line.iter().enumerate() {
            let (x, y) = (x as i32, y as i32);
            let pos = Pos { x, y };
            let height = match height {
                b'a'..=b'z' => height - b'a',
                b'S' => {
                    start = Some(pos);
                    0
                }
                b'E' => {
                    finish = Some(pos);
                    25
                }
                _ => panic!("unknown character: {:?}", height as char),
            };

            heights.insert(pos, height);
        }
    }

    Map {
        heights,
        start: start.expect("Didn't find start location"),
        finish: finish.expect("Didn't find finish location"),
    }
}

pub fn part1(input: &str) -> u32 {
    let map = parse(input);
    shortest_path_distance(&map.heights, map.start, map.finish).expect("no path found")
}

pub fn part2(input: &str) -> u32 {
    let map = parse(input);

    map.heights
        .iter()
        .filter(|(_, &height)| height == 0)
        .filter_map(|(&pos, _)| shortest_path_distance(&map.heights, pos, map.finish))
        .min()
        .expect("no path found")
}

impl Pos {
    fn manhattan_dist(&self, to: &Pos) -> u32 {
        self.x.abs_diff(to.x) + self.y.abs_diff(to.y)
    }

    fn neighbors(&self) -> impl Iterator<Item = Pos> {
        let Pos { x, y } = *self;
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .map(|(x, y)| Pos { x, y })
    }
}

fn shortest_path_distance(heights: &HashMap<Pos, u8>, start: Pos, finish: Pos) -> Option<u32> {
    let mut open_set = BTreeSet::new();
    open_set.insert((start.manhattan_dist(&finish), start));

    // map from every pos to the one immediately preceeding it on the shortest path from start
    // known
    let mut came_from = HashMap::<Pos, Pos>::default();

    // map containing shortest distance we know from start to pos
    let mut g_score = HashMap::default();
    g_score.insert(start, 0);

    // map containing our best guest distance from pos to finish
    let mut f_score = HashMap::default();
    f_score.insert(start, start.manhattan_dist(&finish));

    while let Some((_estimated_dist, current)) = open_set.pop_first() {
        if current == finish {
            return Some(f_score[&current]);
        }

        for neighbor in current
            .neighbors()
            .filter(|neighbor| heights.contains_key(&neighbor))
            .filter(|neighbor| heights[neighbor] as i8 - heights[&current] as i8 <= 1)
        {
            let tentative_g_score = g_score.get(&current).map(|s| s + 1).unwrap_or(u32::MAX);
            if tentative_g_score < g_score.get(&neighbor).copied().unwrap_or(u32::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(
                    neighbor,
                    tentative_g_score + neighbor.manhattan_dist(&finish),
                );
                open_set.insert((f_score[&neighbor], neighbor));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 31);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 29);
    }
}
