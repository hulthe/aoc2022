pub struct Rucksack {
    compartment1: Compartment,
    compartment2: Compartment,
}

pub struct Compartment {
    contents: [u8; 53],
}

pub struct Item {
    priority: u8,
}

pub struct ItemCount {
    item: Item,
    count: u32,
}

pub fn parse(input: &str) -> Vec<Rucksack> {
    input
        .as_bytes()
        .split(|&c| c == b'\n')
        .map(|line| {
            let half = line.len() / 2;
            let first_half = Compartment::from_items(line[..half].iter().copied().map(Item::from));
            let second_half = Compartment::from_items(line[half..].iter().copied().map(Item::from));
            Rucksack {
                compartment1: first_half,
                compartment2: second_half,
            }
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let rucksacks = parse(input);
    rucksacks
        .into_iter()
        .map(|rucksack| {
            // iterate over both compartments in the rucksack
            let comp1 = rucksack.compartment1.items_with_counts();
            let comp2 = rucksack.compartment2.items_with_counts();
            (comp1.zip(comp2))
                // find the item which exist in both compartments
                .find(|(a, b)| a.count > 0 && b.count > 0)
                .map(|(ic, _)| ic.item.priority as u32)
                .unwrap_or(0)
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let rucksacks = parse(input);
    rucksacks
        .array_chunks()
        .map(|[sack1, sack2, sack3]| {
            // compare contents of each rucksack
            (sack1.contents().items_with_counts())
                .zip(sack2.contents().items_with_counts())
                .zip(sack3.contents().items_with_counts())
                .map(|((a, b), c)| [a, b, c])
                // get the first item which exist in all 3 rucksacks
                .find(|item_counts| item_counts.iter().all(|ic| ic.count > 0))
                // take the priority
                .map(|item_counts| item_counts[0].item.priority as u32)
                .unwrap_or(0)
        })
        .sum()
}

impl Rucksack {
    /// Get the entire contents of the rucksack
    fn contents(&self) -> Compartment {
        let contents = (self.compartment1.contents)
            .zip(self.compartment2.contents)
            .map(|(a, b)| a + b);

        Compartment { contents }
    }
}

impl Compartment {
    fn from_items<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Item>,
    {
        let mut contents = [0u8; 53];
        for item in iter {
            contents[item.priority as usize] += 1;
        }

        Compartment { contents }
    }

    fn items_with_counts(&self) -> impl Iterator<Item = ItemCount> + '_ {
        self.contents
            .iter()
            .enumerate()
            .map(|(priority, &count)| ItemCount {
                item: Item {
                    priority: priority as u8,
                },
                count: count as u32,
            })
    }
}

impl From<u8> for Item {
    fn from(c: u8) -> Item {
        let priority = match c {
            b'a'..=b'z' => 1 + c - b'a',
            b'A'..=b'Z' => 27 + c - b'A',
            _ => panic!("Item out of range"),
        };

        Item { priority }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 157);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 70);
    }
}
