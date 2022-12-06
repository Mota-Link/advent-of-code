use std::fs::read_to_string;

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|depth| depth.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|couple| couple[0] < couple[1])
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|depth| depth.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|window| window.into_iter().sum::<u32>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|couple| couple[0] < couple[1])
        .count()
}

pub fn print_answer() {
    let input = read_to_string("./src/aoc_2021/data/input_2021_01").unwrap();
    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&input.trim()),
        part_2(&input.trim())
    )
}

#[test]
fn part_1_test() {
    let input: &str = "199\n\
                       200\n\
                       208\n\
                       210\n\
                       200\n\
                       207\n\
                       240\n\
                       269\n\
                       260\n\
                       263";
    assert_eq!(part_1(input), 7);
}

#[test]
fn part_2_test() {
    let input: &str = "199\n\
                       200\n\
                       208\n\
                       210\n\
                       200\n\
                       207\n\
                       240\n\
                       269\n\
                       260\n\
                       263";
    assert_eq!(part_2(input), 5);
}