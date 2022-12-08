use std::{collections::HashMap, fs::read_to_string};

fn part_1(input: &str) -> u32 {
    let (hsm, len) = input
        .lines()
        .map(|line| line.chars().rev().enumerate())
        .fold((HashMap::new(), 0u32), |mut init, item| {
            for (idx, ch) in item {
                if ch == '1' {
                    let count = init.0.entry(idx).or_insert(0u32);
                    *count += 1;
                }
            }
            init.1 += 1;
            init
        });

    let gamma: u32 = hsm
        .iter()
        .map(|(idx, count)| {
            if *count >= len / 2 {
                2u32.pow(*idx as u32)
            } else {
                0
            }
        })
        .sum();

    let epsilon: u32 = hsm
        .iter()
        .map(|(idx, count)| {
            if *count < len / 2 {
                2u32.pow(*idx as u32)
            } else {
                0
            }
        })
        .sum();

    gamma * epsilon
}

fn part_2(input: &str) -> u32 {
    let vector = input
        .lines()
        .map(|line| line.chars().rev().enumerate().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    oxygen_generator_rating(vector.clone()) * co2_generator_rating(vector)
}

fn oxygen_generator_rating(mut vector: Vec<Vec<(usize, char)>>) -> u32 {
    let mut element_count = vector.len();
    let bit_count = vector[0].len();
    for i in (0..bit_count).rev() {
        vector.sort_unstable_by_key(|element| element[i].1);
        let zero_element_count = vector.partition_point(|x| x[i].1 == '0');
        if zero_element_count as f32 > element_count as f32 / 2.0 {
            vector.retain(|x| x[i].1 == '0');
        } else {
            vector.retain(|x| x[i].1 == '1');
        }

        element_count = vector.len();
        if element_count == 1 {
            break;
        }
    }

    vector[0]
        .iter()
        .map(|(idx, count)| count.to_digit(2).unwrap() * 2u32.pow(*idx as u32))
        .sum()
}

fn co2_generator_rating(mut vector: Vec<Vec<(usize, char)>>) -> u32 {
    let mut element_count = vector.len();
    let bit_count = vector[0].len();
    for i in (0..bit_count).rev() {
        vector.sort_unstable_by_key(|element| element[i].1);
        let zero_element_count = vector.partition_point(|x| x[i].1 == '0');
        if zero_element_count as f32 <= element_count as f32 / 2.0 {
            vector.retain(|x| x[i].1 == '0');
        } else {
            vector.retain(|x| x[i].1 == '1');
        }

        element_count = vector.len();
        if element_count == 1 {
            break;
        }
    }

    vector[0]
        .iter()
        .map(|(idx, count)| count.to_digit(2).unwrap() * 2u32.pow(*idx as u32))
        .sum()
}

pub fn print_answer() {
    let input = read_to_string("./src/aoc_2021/data/input_2021_03").unwrap();
    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&input.trim()),
        part_2(&input.trim())
    )
}

#[test]
fn part_1_test() {
    let input: &str = "00100\n\
                       11110\n\
                       10110\n\
                       10111\n\
                       10101\n\
                       01111\n\
                       00111\n\
                       11100\n\
                       10000\n\
                       11001\n\
                       00010\n\
                       01010";
    assert_eq!(part_1(input), 198);
}

#[test]
fn part_2_test() {
    let input: &str = "00100\n\
                       11110\n\
                       10110\n\
                       10111\n\
                       10101\n\
                       01111\n\
                       00111\n\
                       11100\n\
                       10000\n\
                       11001\n\
                       00010\n\
                       01010";
    assert_eq!(part_2(input), 230);
}
