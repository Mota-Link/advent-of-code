use std::fs::read_to_string;

pub fn part_1() -> usize {
    read_to_string("./src/aoc_2022/input/input_2022_04")
        .unwrap()
        .lines()
        .filter(|s| {
            let (r1_start, r1_end, r2_start, r2_end) = parser(s);
            (r1_start >= r2_start && r1_end <= r2_end) || (r1_start <= r2_start && r1_end >= r2_end)
        })
        .count()
}

pub fn part_2() -> usize {
    read_to_string("./src/aoc_2022/input/input_2022_04")
        .unwrap()
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
