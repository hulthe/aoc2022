#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Rps {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[repr(u8)]
pub enum Xyz {
    X,
    Y,
    Z,
}

pub fn parse(input: &str) -> impl Iterator<Item = (Rps, Xyz)> + '_ {
    input
        .as_bytes()
        .array_chunks()
        .map(|[opponents_move, _space, xyz, _newline]| {
            let opponents_move = match opponents_move {
                b'A' => Rps::Rock,
                b'B' => Rps::Paper,
                b'C' => Rps::Scissor,
                &c => panic!("parse failed: expected [ABC], got {}", c as char),
            };

            let xyz = match xyz {
                b'X' => Xyz::X,
                b'Y' => Xyz::Y,
                b'Z' => Xyz::Z,
                &c => panic!("parse failed: expected [XYZ], got {}", c as char),
            };

            (opponents_move, xyz)
        })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .map(|(opponents_move, my_move)| {
            let my_move: Rps = my_move.into();
            let win_score = duel(my_move, opponents_move);
            let move_score = my_move as usize;
            win_score + move_score
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .map(|(opponents_move, outcome)| {
            let outcome: Outcome = outcome.into();

            let i_must_choose = match (opponents_move, outcome) {
                (Rps::Rock, Outcome::Win) => Rps::Paper,
                (Rps::Paper, Outcome::Win) => Rps::Scissor,
                (Rps::Scissor, Outcome::Win) => Rps::Rock,
                (Rps::Rock, Outcome::Loss) => Rps::Scissor,
                (Rps::Paper, Outcome::Loss) => Rps::Rock,
                (Rps::Scissor, Outcome::Loss) => Rps::Paper,
                (_, Outcome::Draw) => opponents_move,
            };

            let win_score = outcome as usize;
            let move_score = i_must_choose as usize;

            win_score + move_score
        })
        .sum()
}

fn duel(my_move: Rps, opponents_move: Rps) -> usize {
    use Rps::*;
    let outcome = match (my_move, opponents_move) {
        (Rock, Scissor) => Outcome::Win,
        (Scissor, Paper) => Outcome::Win,
        (Paper, Rock) => Outcome::Win,
        (Scissor, Rock) => Outcome::Loss,
        (Paper, Scissor) => Outcome::Loss,
        (Rock, Paper) => Outcome::Loss,
        _ => Outcome::Draw,
    };

    outcome as usize
}

impl From<Xyz> for Rps {
    fn from(xyz: Xyz) -> Self {
        match xyz {
            Xyz::X => Rps::Rock,
            Xyz::Y => Rps::Paper,
            Xyz::Z => Rps::Scissor,
        }
    }
}

impl From<Xyz> for Outcome {
    fn from(xyz: Xyz) -> Self {
        match xyz {
            Xyz::X => Outcome::Loss,
            Xyz::Y => Outcome::Draw,
            Xyz::Z => Outcome::Win,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 15);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 12);
    }
}
