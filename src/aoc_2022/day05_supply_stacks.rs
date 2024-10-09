use std::{fs::read_to_string, sync::LazyLock};

static STACKS: LazyLock<Vec<Vec<char>>> = LazyLock::new(|| {
    vec![
        vec!['S', 'T', 'H', 'F', 'W', 'R'],
        vec!['S', 'G', 'D', 'Q', 'W'],
        vec!['B', 'T', 'W'],
        vec!['D', 'R', 'W', 'T', 'N', 'Q', 'Z', 'J'],
        vec!['F', 'B', 'H', 'G', 'L', 'V', 'T', 'Z'],
        vec!['L', 'P', 'T', 'C', 'V', 'B', 'S', 'G'],
        vec!['Z', 'B', 'R', 'T', 'W', 'G', 'P'],
        vec!['N', 'G', 'M', 'T', 'C', 'J', 'R'],
        vec!['L', 'G', 'B', 'W'],
    ]
});

fn part_1(input: &str, mut stacks: Vec<Vec<char>>) -> String {
    input
        .lines()
        .map(|i| parser(i))
        .for_each(|(mov, from, to)| {
            for _ in 0..mov {
                let ch = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(ch);
            }
        });

    get_stacks_top(stacks)
}

fn part_2(input: &str, mut stacks: Vec<Vec<char>>) -> String {
    input
        .lines()
        .map(|i| parser(i))
        .for_each(|(mov, from, to)| {
            let boundary = stacks[from - 1].len() - mov;
            let mut drain = stacks[from - 1].drain(boundary..).collect();
            stacks[to - 1].append(&mut drain)
        });

    get_stacks_top(stacks)
}

fn parser(input: &str) -> (usize, usize, usize) {
    let mut nums = input
        .split_whitespace()
        .filter_map(|item| item.parse::<usize>().ok());

    (
        nums.next().unwrap(),
        nums.next().unwrap(),
        nums.next().unwrap(),
    )
}

fn get_stacks_top(stacks: Vec<Vec<char>>) -> String {
    stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect()
}

pub fn print_answer() {
    let input = read_to_string("./src/aoc_2022/data/input_2022_05").unwrap();
    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&input.trim(), STACKS.clone()),
        part_2(&input.trim(), STACKS.clone())
    )
}

#[test]
fn part_1_test() {
    let input: &str = "move 1 from 2 to 1\n\
                       move 3 from 1 to 3\n\
                       move 2 from 2 to 1\n\
                       move 1 from 1 to 2";
    let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    assert_eq!(part_1(input, stacks), "CMZ");
}

#[test]
fn part_2_test() {
    let input: &str = "move 1 from 2 to 1\n\
                       move 3 from 1 to 3\n\
                       move 2 from 2 to 1\n\
                       move 1 from 1 to 2";
    let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    assert_eq!(part_2(input, stacks), "MCD");
}
