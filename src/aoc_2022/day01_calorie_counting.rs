use std::fs::read_to_string;

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|s| s.trim())
        .collect::<Vec<_>>()
        .split(|s| s.is_empty())
        .map(|calories| {
            calories
                .into_iter()
                .map(|ca| ca.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap()
}

fn part_2(input: &str) -> u64 {
    let mut calories = input
        .lines()
        .map(|s| s.trim())
        .collect::<Vec<_>>()
        .split(|s| s.is_empty())
        .map(|calories| {
            calories
                .into_iter()
                .map(|ca| ca.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .collect::<Vec<_>>();
    calories.sort();
    calories.into_iter().rev().take(3).sum()
}

pub fn day01_answer() {
    let input = read_to_string("./src/aoc_2022/input/input_2022_01").unwrap();
    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&input.trim()),
        part_2(&input.trim())
    )
}

#[test]
fn part_1_test() {
    let input: &str = "1000\n\
                       2000\n\
                       3000\n\
                           \n\
                       4000\n\
                           \n\
                       5000\n\
                       6000\n\
                           \n\
                       7000\n\
                       8000\n\
                       9000\n\
                           \n\
                       10000";
    assert_eq!(part_1(input), 24000);
}

#[test]
fn part_2_test() {
    let input: &str = "1000\n\
                       2000\n\
                       3000\n\
                           \n\
                       4000\n\
                           \n\
                       5000\n\
                       6000\n\
                           \n\
                       7000\n\
                       8000\n\
                       9000\n\
                           \n\
                       10000";
    assert_eq!(part_2(input), 45000);
}
