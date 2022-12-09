use crate::util::HashMap;
use std::iter::Peekable;

#[derive(Debug)]
pub enum DirEntry<'a> {
    File(File),
    Dir(Dir<'a>),
}

#[derive(Debug)]
pub struct File {
    size: usize,
}

#[derive(Debug, Default)]
pub struct Dir<'a> {
    entries: HashMap<&'a str, DirEntry<'a>>,
}

pub fn parse(input: &str) -> Dir {
    fn parse_dir<'a>(lines: &mut Peekable<impl Iterator<Item = &'a str>>) -> Dir<'a> {
        let mut dir = Dir::default();

        while let Some(line) = lines.next() {
            if let Some(cmd) = line.strip_prefix("$ ") {
                if cmd == "ls" {
                } else if let Some(dirname) = cmd.strip_prefix("cd ") {
                    match dirname {
                        ".." => return dir,
                        dirname => {
                            dir.entries.insert(dirname, DirEntry::Dir(parse_dir(lines)));
                        }
                    }
                } else {
                    panic!()
                }
            } else {
                if let Some(dirname) = line.strip_prefix("dir ") {
                    dir.entries.insert(dirname, DirEntry::Dir(Dir::default()));
                } else {
                    let (size, name) = line.split_once(' ').unwrap();
                    let size = size.parse().unwrap();
                    dir.entries.insert(name, DirEntry::File(File { size }));
                }
            }
        }

        dir
    }

    let mut lines = input.lines().peekable();

    assert_eq!(lines.next(), Some("$ cd /"));
    parse_dir(&mut lines)
}

pub fn part1(input: &str) -> usize {
    let fs = parse(input);
    let mut sum = 0;
    find_files_with_sizes(&fs, &mut |s| s <= 100000, &mut sum);
    sum
}

pub fn part2(input: &str) -> usize {
    let fs = parse(input);
    let total_size = 70000000;
    let mut size_of_every_dir = vec![];
    let used_space = find_files_with_sizes(
        &fs,
        &mut |s| {
            size_of_every_dir.push(s);
            false
        },
        &mut 0,
    );
    let required_space = 30000000 - (total_size - used_space);
    size_of_every_dir
        .into_iter()
        .filter(|&s| s >= required_space)
        .min()
        .unwrap_or(0)
}

fn find_files_with_sizes(dir: &Dir, f: &mut impl FnMut(usize) -> bool, sum: &mut usize) -> usize {
    let dir_size = dir
        .entries
        .values()
        .map(|entry| match entry {
            &DirEntry::File(File { size }) => size,
            DirEntry::Dir(dir) => find_files_with_sizes(dir, f, sum),
        })
        .sum();

    if f(dir_size) {
        *sum += dir_size;
    }

    dir_size
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 95437);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 24933642);
    }
}
