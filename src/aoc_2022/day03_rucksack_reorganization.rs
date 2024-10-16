use std::{collections::HashSet, fs::read_to_string};

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(s1, s2)| {
            let s1 = s1.chars().collect::<HashSet<_>>();
            let s2 = s2.chars().collect::<HashSet<_>>();

            match s1.intersection(&s2).next() {
                Some(&i @ 'a'..='z') => i as u32 - 96,
                Some(&i @ 'A'..='Z') => i as u32 - 38,
                _ => 0,
            }
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let s1 = chunk[0].chars().collect::<HashSet<_>>();
            let s2 = chunk[1].chars().collect::<HashSet<_>>();
            let s3 = chunk[2].chars().collect::<HashSet<_>>();

            match s1
                .intersection(&s2)
                .map(|&s| s)
                .collect::<HashSet<_>>()
                .intersection(&s3)
                .next()
            {
                Some(&i @ 'a'..='z') => i as u32 - 96,
                Some(&i @ 'A'..='Z') => i as u32 - 38,
                _ => 0,
            }
        })
        .sum()
}

pub fn print_answer() {
    let input = read_to_string("./src/aoc_2022/data/input_2022_03").unwrap();
    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&input.trim()),
        part_2(&input.trim())
    )
}

#[test]
fn part_1_test() {
    let input: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                       jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                       PmmdzqPrVvPwwTWBwg\n\
                       wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                       ttgJtRGJQctTZtZT\n\
                       CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(part_1(input), 157);
}

#[test]
fn part_2_test() {
    let input: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                       jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                       PmmdzqPrVvPwwTWBwg\n\
                       wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                       ttgJtRGJQctTZtZT\n\
                       CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(part_2(input), 70);
}
