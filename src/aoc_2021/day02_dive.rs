use std::fs::read_to_string;

struct Position {
    horizontal_position: u32,
    depth: u32,
    aim: u32,
}

fn part_1(input: &str) -> u32 {
    let mut pos = Position {
        horizontal_position: 0,
        depth: 0,
        aim: 0,
    };

    input
        .lines()
        .map(|command| parse(command))
        .for_each(|command| match command.0 {
            "forward" => pos.horizontal_position += command.1,
            "down" => pos.depth += command.1,
            "up" => pos.depth -= command.1,
            _ => (),
        });
    pos.horizontal_position * pos.depth
}

fn part_2(input: &str) -> u32 {
    let mut pos = Position {
        horizontal_position: 0,
        depth: 0,
        aim: 0,
    };

    input
        .lines()
        .map(|command| parse(command))
        .for_each(|command| match command.0 {
            "forward" => {
                pos.horizontal_position += command.1;
                pos.depth += command.1 * pos.aim;
            }
            "down" => pos.aim += command.1,
            "up" => pos.aim -= command.1,
            _ => (),
        });
    pos.horizontal_position * pos.depth
}

fn parse(input: &str) -> (&str, u32) {
    let mut command = input.split_whitespace();
    (
        command.next().unwrap(),
        command.next().unwrap().parse().unwrap(),
    )
}

pub fn print_answer() {
    let input = read_to_string("./src/aoc_2021/data/input_2021_02").unwrap();
    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&input.trim()),
        part_2(&input.trim())
    )
}

#[test]
fn part_1_test() {
    let input: &str = "forward 5\n\
                       down 5\n\
                       forward 8\n\
                       up 3\n\
                       down 8\n\
                       forward 2";
    assert_eq!(part_1(input), 150);
}

#[test]
fn part_2_test() {
    let input: &str = "forward 5\n\
                       down 5\n\
                       forward 8\n\
                       up 3\n\
                       down 8\n\
                       forward 2";
    assert_eq!(part_2(input), 900);
}
