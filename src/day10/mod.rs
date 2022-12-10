pub enum Instruction {
    NoOp,
    AddX(i32),
}

impl Instruction {
    pub fn delay(&self) -> i32 {
        match self {
            Instruction::NoOp => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.split(' '))
        .filter_map(|mut words| {
            Some(match words.next()? {
                "noop" => Instruction::NoOp,
                "addx" => Instruction::AddX(words.next()?.parse().ok()?),
                unknown => panic!("unknown instruction: {unknown}"),
            })
        })
        .collect()
}
pub fn part1(input: &str) -> i32 {
    let instructions = parse(input);
    let mut cycle = 1;
    let mut reg_x: i32 = 1;
    let mut sum = 0;

    for instruction in instructions {
        for _ in 0..instruction.delay() {
            if (cycle - 20) % 40 == 0 {
                sum += cycle * reg_x;
            }
            cycle += 1;
        }

        match instruction {
            Instruction::NoOp => {}
            Instruction::AddX(add) => reg_x += add,
        }
    }

    sum
}

pub fn part2(input: &str) -> String {
    let instructions = parse(input);
    const CRT_W: usize = 40;
    const CRT_H: usize = 6;
    let mut crt_out = String::with_capacity((CRT_W + 1) * CRT_H);
    let mut crt_x: usize = 0;
    let mut reg_x: i32 = 1;
    for instruction in instructions {
        for _ in 0..instruction.delay() {
            let sprit_pos = reg_x - 1..=reg_x + 1;
            if sprit_pos.contains(&(crt_x as i32)) {
                crt_out.push('#');
            } else {
                crt_out.push('.');
            }

            crt_x += 1;
            if crt_x >= CRT_W {
                crt_x = 0;
                crt_out.push('\n');
            }
        }

        match instruction {
            Instruction::NoOp => {}
            Instruction::AddX(add) => reg_x += add,
        }
    }

    crt_out
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 13140);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(
            part2(input),
            "##..##..##..##..##..##..##..##..##..##..\n\
             ###...###...###...###...###...###...###.\n\
             ####....####....####....####....####....\n\
             #####.....#####.....#####.....#####.....\n\
             ######......######......######......####\n\
             #######.......#######.......#######.....\n"
        );
    }
}
