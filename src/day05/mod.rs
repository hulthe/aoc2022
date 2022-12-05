use crate::util::get_2_mut;

pub type Crate = u8;

pub struct Input {
    stacks: Vec<Vec<Crate>>,
    instructions: Vec<Instruction>,
}

pub struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

pub fn parse(input: &str) -> Input {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut stack_iter = stacks.lines().rev();
    let stack_count = stack_iter.next().unwrap().as_bytes().chunks(4).count();
    let mut stacks = vec![vec![]; stack_count];

    for stack in stack_iter {
        for (i, item) in stack.as_bytes().chunks(4).enumerate() {
            let item = item[1];
            if item != b' ' {
                stacks[i].push(item);
            }
        }
    }

    let instructions = instructions
        .lines()
        .filter_map(|line| {
            let line = line.strip_prefix("move ")?;
            let (amount, line) = line.split_once(" from ")?;
            let (from, to) = line.split_once(" to ")?;
            Some(Instruction {
                amount: amount.parse().ok()?,
                from: from.parse::<usize>().ok()? - 1,
                to: to.parse::<usize>().ok()? - 1,
            })
        })
        .collect();

    Input {
        stacks,
        instructions,
    }
}

pub fn part1(input: &str) -> String {
    let mut input = parse(input);
    for Instruction { amount, from, to } in input.instructions {
        let [from, to] = get_2_mut(&mut input.stacks, from, to);
        let start = from.len() - amount;
        from[start..].reverse();
        to.extend(from.drain(start..));
    }

    display_stacks(&input.stacks)
}

pub fn part2(input: &str) -> String {
    let mut input = parse(input);

    for Instruction { amount, from, to } in input.instructions {
        let [from, to] = get_2_mut(&mut input.stacks, from, to);
        let start = from.len() - amount;
        to.extend(from.drain(start..));
    }

    display_stacks(&input.stacks)
}

fn display_stacks(stacks: &Vec<Vec<Crate>>) -> String {
    let mut out = String::new();
    for stack in stacks {
        let &item = stack.last().unwrap_or(&b' ');
        out.push(item as char);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::{parse, part1, part2};

    #[test]
    pub fn test_parse() {
        let input = parse(include_str!("test-input"));
        assert_eq!(
            input.stacks,
            vec![vec![b'Z', b'N'], vec![b'M', b'C', b'D'], vec![b'P']],
        );
    }

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), "CMZ");
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), "MCD");
    }
}
