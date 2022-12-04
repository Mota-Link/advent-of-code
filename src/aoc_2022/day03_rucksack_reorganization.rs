use std::{collections::HashSet, fs::read_to_string};

pub fn part_1() -> u32 {
    read_to_string("./src/aoc_2022/input/input_2022_03")
        .unwrap()
        .lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(s1, s2)| {
            /* Version 1.0 */
            // let mut s1 = s1.chars().collect::<Vec<_>>();
            // s1.sort();
            // s1.dedup();

            // let mut s2 = s2.chars().collect::<Vec<_>>();
            // s2.sort();
            // s2.dedup();

            // let mut hmap: HashMap<char, u32> = HashMap::new();
            // for i in s1.into_iter().chain(s2) {
            //     let counter = hmap.entry(i).or_insert(0);
            //     *counter += 1;
            // }

            // match hmap.into_iter().find(|(_, ctr)| *ctr != 1) {
            //     Some((i @ 'a'..='z', _)) => i as u32 - 96,
            //     Some((i @ 'A'..='Z', _)) => i as u32 - 38,
            //     _ => 0,
            // }

            /* Version 2.0 */
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

pub fn part_2() -> u32 {
    read_to_string("./src/aoc_2022/input/input_2022_03")
        .unwrap()
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            /* Version 1.0 */
            // let mut s1 = chunk[0].chars().collect::<Vec<_>>();
            // s1.sort();
            // s1.dedup();

            // let mut s2 = chunk[1].chars().collect::<Vec<_>>();
            // s2.sort();
            // s2.dedup();

            // let mut s3 = chunk[2].chars().collect::<Vec<_>>();
            // s3.sort();
            // s3.dedup();

            // let mut hmap: HashMap<char, u32> = HashMap::new();
            // for i in s1.into_iter().chain(s2).chain(s3) {
            //     let counter = hmap.entry(i).or_insert(0);
            //     *counter += 1;
            // }

            // match hmap.into_iter().find(|(_, ctr)| *ctr == 3) {
            // Some((i @ 'a'..='z', _)) => i as u32 - 96,
            // Some((i @ 'A'..='Z', _)) => i as u32 - 38,
            // _ => 0,
            // }

            /* Version 2.0 */
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
