use itertools::{Itertools};

fn parse_input() -> Vec<Vec<char>> {
    let content = std::fs::read_to_string("input/day03.txt").unwrap();

    content.lines().map(|line| line.chars().collect()).collect()
}

fn get_char_priority(ch: &char) -> u16 {
    return if &'A' <= ch && ch <= &'Z' {
        *ch as u16 - 38
    } else {
        *ch as u16 - 96
    }
}

pub fn part_one() {
    let before = std::time::Instant::now();

    let lines = parse_input();

    let mut sum = 0;
    for line in lines {
        let mid = line.len() / 2;
        let first: Vec<&char> = line.iter().take(mid).collect();
        let second: Vec<&char> = line.iter().skip(mid).collect();

        let intersection: Vec<&char> = first
            .iter()
            .filter(|&&char| second.contains(&char))
            .cloned()
            .dedup()
            .collect();

        let intersection_value: u16 = intersection
            .iter()
            .map(|&char| get_char_priority(char))
            .sum();

        sum += intersection_value;
    }

    println!("[{:.2?}]\t Day 3 part 1 result: {}", before.elapsed(), sum);
}

pub fn part_two() {
    let before = std::time::Instant::now();

    let lines = parse_input();
    let groups: Vec<Vec<Vec<char>>> = lines
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect();

    let mut sum = 0;
    for group in groups {
        let intersection: Vec<char> = group[0]
            .iter()
            .filter(|char| group[1].contains(char) && group[2].contains(char))
            .cloned()
            .dedup()
            .collect();

        let intersection_value: u16 = intersection
            .iter()
            .map(|char| get_char_priority(char))
            .sum();

        sum += intersection_value;
    }

    println!("[{:.2?}]\t Day 3 part 2 result: {}", before.elapsed(), sum);
}