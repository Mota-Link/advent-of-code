use std::fs::read_to_string;

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|s| {
            let (r1_start, r1_end, r2_start, r2_end) = parser(s);
            (r1_start >= r2_start && r1_end <= r2_end) || (r1_start <= r2_start && r1_end >= r2_end)
        })
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|s| {
            let (r1_start, r1_end, r2_start, r2_end) = parser(s);
            (r1_start >= r2_start && r1_start <= r2_end)
                || (r2_start >= r1_start && r2_start <= r1_end)
        })
        .count()
}

fn parser(s: &str) -> (u8, u8, u8, u8) {
    let (r1, r2) = s.split_once(',').unwrap();
    let (r1_start, r1_end) = r1.split_once('-').unwrap();
    let (r2_start, r2_end) = r2.split_once('-').unwrap();
    (
        r1_start.parse().unwrap(),
        r1_end.parse().unwrap(),
        r2_start.parse().unwrap(),
        r2_end.parse().unwrap(),
    )
}

pub fn day04_answer() {
    let input = read_to_string("./src/aoc_2022/input/input_2022_04").unwrap();
    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&input.trim()),
        part_2(&input.trim())
    )
}

#[test]
fn part_1_test() {
    let input = "2-4,6-8\n\
                       2-3,4-5\n\
                       5-7,7-9\n\
                       2-8,3-7\n\
                       6-6,4-6\n\
                       2-6,4-8";
    assert_eq!(part_1(input), 2);
}

#[test]
fn part_2_test() {
    let input = "2-4,6-8\n\
                       2-3,4-5\n\
                       5-7,7-9\n\
                       2-8,3-7\n\
                       6-6,4-6\n\
                       2-6,4-8";
    assert_eq!(part_2(input), 4);
}
