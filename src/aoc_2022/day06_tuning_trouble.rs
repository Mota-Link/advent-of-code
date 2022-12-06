use std::fs::read_to_string;

fn part_1(input: &str) -> usize {
    detect_start_marker(input, 4)
}

fn part_2(input: &str) -> usize {
    detect_start_marker(input, 14)
}

fn detect_start_marker(input: &str, marker_len: usize) -> usize {
    input
        .as_bytes()
        .windows(marker_len)
        .enumerate()
        .find_map(|(idx, window)| {
            let mut window = window.to_vec();
            window.sort();
            window.dedup();
            if window.len() == marker_len {
                Some(idx + marker_len)
            } else {
                None
            }
        })
        .unwrap()
}

pub fn print_answer() {
    let input = read_to_string("./src/aoc_2022/data/input_2022_06").unwrap();
    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&input.trim()),
        part_2(&input.trim())
    )
}

#[test]
fn part_1_test() {
    let input_0: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let input_1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let input_2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    let input_3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let input_4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(part_1(input_0), 7);
    assert_eq!(part_1(input_1), 5);
    assert_eq!(part_1(input_2), 6);
    assert_eq!(part_1(input_3), 10);
    assert_eq!(part_1(input_4), 11);
}

#[test]
fn part_2_test() {
    let input_0: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let input_1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let input_2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    let input_3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let input_4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(part_2(input_0), 19);
    assert_eq!(part_2(input_1), 23);
    assert_eq!(part_2(input_2), 23);
    assert_eq!(part_2(input_3), 29);
    assert_eq!(part_2(input_4), 26);
}
