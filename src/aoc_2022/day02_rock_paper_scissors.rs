use lazy_static::lazy_static;
use std::{collections::HashMap, fs::read_to_string};

lazy_static! {
    static ref SCORE_PART_1: HashMap<&'static str, u32> = HashMap::from([
        ("B X", 1),
        ("C Y", 2),
        ("A Z", 3),
        ("A X", 4),
        ("B Y", 5),
        ("C Z", 6),
        ("C X", 7),
        ("A Y", 8),
        ("B Z", 9),
    ]);
    static ref SCORE_PART_2: HashMap<&'static str, u32> = HashMap::from([
        ("A X", 3),
        ("B X", 1),
        ("C X", 2),
        ("A Y", 4),
        ("B Y", 5),
        ("C Y", 6),
        ("A Z", 8),
        ("B Z", 9),
        ("C Z", 7),
    ]);
}

fn part_1(input: &str) -> u32 {
    input.lines().map(|s| *SCORE_PART_1.get(s).unwrap()).sum()
}
fn part_2(input: &str) -> u32 {
    input.lines().map(|s| *SCORE_PART_2.get(s).unwrap()).sum()
}

pub fn day02_answer() {
    let input = read_to_string("./src/aoc_2022/input/input_2022_02").unwrap();
    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&input.trim()),
        part_2(&input.trim())
    )
}

#[test]
fn part_1_test() {
    let input: &str = "A Y\n\
                       B X\n\
                       C Z";

    assert_eq!(part_1(input), 15)
}

#[test]
fn part_2_test() {
    let input: &str = "A Y\n\
                       B X\n\
                       C Z";

    assert_eq!(part_2(input), 12)
}
