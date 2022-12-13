#[derive(Clone, PartialEq, Ord, Eq)]
pub enum Item {
    List(Vec<Item>),
    Num(i32),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Item::List(vec1), Item::List(vec2)) => vec1.partial_cmp(vec2),
            (Item::List(vec), Item::Num(n)) => vec.as_slice().partial_cmp(&[Item::Num(*n)]),
            (Item::Num(n), Item::List(vec)) => [Item::Num(*n)][..].partial_cmp(vec),
            (Item::Num(n1), Item::Num(n2)) => n1.partial_cmp(n2),
        }
    }
}

pub fn parse(input: &str) -> Vec<Vec<Item>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .flat_map(parse_list)
        .collect()
}

fn parse_item(s: &str) -> Item {
    parse_list(s)
        .map(Item::List)
        .unwrap_or_else(|| Item::Num(s.parse().expect("failed to parse")))
}

fn parse_list(s: &str) -> Option<Vec<Item>> {
    if s == "[]" {
        return Some(vec![]);
    }
    let s = s.strip_prefix("[")?.strip_suffix("]")?;
    let mut depth = 0;
    let list = s
        .split(|c| match c {
            ',' if depth == 0 => true,
            '[' => {
                depth += 1;
                false
            }
            ']' => {
                depth -= 1;
                false
            }
            _ => false,
        })
        .map(parse_item)
        .collect();

    Some(list)
}

pub fn part1(input: &str) -> usize {
    let pairs = parse(input);
    pairs
        .into_iter()
        .array_chunks()
        .enumerate()
        .filter(|(_, [a, b])| a < b)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut packets = parse(input);
    let div1 = vec![Item::List(vec![Item::Num(2)])];
    let div2 = vec![Item::List(vec![Item::Num(6)])];
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();
    let i1 = packets
        .iter()
        .enumerate()
        .find_map(|(i, packet)| (packet == &div1).then(|| i + 1))
        .unwrap();
    let i2 = packets
        .iter()
        .enumerate()
        .find_map(|(i, packet)| (packet == &div2).then(|| i + 1))
        .unwrap();
    i1 * i2
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 13);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 1337);
    }
}
