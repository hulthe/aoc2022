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

pub const CRT_W: usize = 40;
pub const CRT_H: usize = 6;
pub const CRT_LETTER_W: usize = CRT_W / 8;

pub fn part2(input: &str) -> String {
    let instructions = parse(input);
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

    parse_crt(crt_out)
}

/// Try to parse the output of the CRT display as text
fn parse_crt(crt: String) -> String {
    let mut parsed = String::with_capacity(8);
    'crt: for start in (0..CRT_W).step_by(CRT_LETTER_W) {
        'letters: for &(letter, bitmap) in BITMAPS {
            for (crt_line, bitmap_line) in crt.lines().zip(bitmap.lines()) {
                let x_range = start..start + CRT_LETTER_W;
                if &crt_line[x_range] != bitmap_line {
                    continue 'letters;
                }
            }

            parsed.push(letter);
            continue 'crt;
        }

        // failed to parse letter, give up and return crt output
        return crt;
    }

    parsed
}

/// Bitmaps representing the CRT font
const BITMAPS: &[(char, &str)] = &[
    (
        'F',
        "####.\n\
         #....\n\
         ###..\n\
         #....\n\
         #....\n\
         #....",
    ),
    (
        'Z',
        "####.\n\
         ...#.\n\
         ..#..\n\
         .#...\n\
         #....\n\
         ####.",
    ),
    (
        'B',
        "###..\n\
         #..#.\n\
         ###..\n\
         #..#.\n\
         #..#.\n\
         ###..",
    ),
    (
        'P',
        "###..\n\
         #..#.\n\
         #..#.\n\
         ###..\n\
         #....\n\
         #....",
    ),
];

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
