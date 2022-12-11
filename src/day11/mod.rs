use std::collections::VecDeque;

pub struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test_div_by: i64,
    if_true: usize,
    if_false: usize,
}

pub enum Operation {
    Mul(i64),
    Add(i64),
    Square,
}

pub fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let [_, starting_items, operation, test, if_true, if_false] =
                monkey.lines().array_chunks().next().unwrap();

            let starting_items = starting_items.strip_prefix("  Starting items: ").unwrap();
            let items = starting_items
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect();

            let operation = operation.strip_prefix("  Operation: new = ").unwrap();
            let operation = if operation == "old * old" {
                Operation::Square
            } else if let Some(mul) = operation.strip_prefix("old * ") {
                Operation::Mul(mul.parse().unwrap())
            } else if let Some(add) = operation.strip_prefix("old + ") {
                Operation::Add(add.parse().unwrap())
            } else {
                panic!()
            };

            let test_div_by = test.strip_prefix("  Test: divisible by ").unwrap();
            let test_div_by = test_div_by.parse().unwrap();

            let if_true = if_true
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();
            let if_false = if_false
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            Monkey {
                items,
                operation,
                test_div_by,
                if_true,
                if_false,
            }
        })
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let mut monkeys = parse(input);
    monkey_business(&mut monkeys, 20, |item| item / 3)
}

pub fn part2(input: &str) -> u64 {
    let mut monkeys = parse(input);
    let test_product: i64 = monkeys.iter().map(|monkey| monkey.test_div_by).product();
    monkey_business(&mut monkeys, 10000, |item| item % test_product)
}

pub fn monkey_business(
    monkeys: &mut [Monkey],
    rounds: usize,
    decrease_worry: impl Fn(i64) -> i64,
) -> u64 {
    let mut activity = vec![0; monkeys.len()];
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            activity[i] += monkeys[i].items.len() as u64;
            while let Some(mut item) = monkeys[i].items.pop_front() {
                // monkey inspects item
                match monkeys[i].operation {
                    Operation::Square => item *= item,
                    Operation::Mul(mul) => item *= mul,
                    Operation::Add(add) => item += add,
                }

                // my worry decreases
                item = decrease_worry(item);

                // monkey tests item
                let throw_to = if item % monkeys[i].test_div_by == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };

                // monkey throws item
                monkeys[throw_to].items.push_back(item);
            }
        }
    }

    activity.sort();
    activity.reverse();
    activity[0] * activity[1]
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 10605);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 2713310158);
    }
}
