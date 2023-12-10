use itertools::sorted;

fn parse_input() -> Vec<Vec<i32>> {
    let content = std::fs::read_to_string("input/day01.txt").unwrap();
    let blocks: Vec<&str> = content
        .split("\r\n\r\n")
        .filter(|s| !s.is_empty())
        .collect();

    let seeds: Vec<Vec<i32>> = blocks
        .iter()
        .map(|block| block.split("\r\n").map(|line| line.parse().unwrap()).collect())
        .collect();

    seeds
}

pub fn part_one() {
    let before = std::time::Instant::now();

    let elves = parse_input();

    let max_calories: i32 = elves.iter().map(|elf| elf.iter().sum()).max().unwrap();

    println!("[{:.2?}]\t Day 1 part 1 result: {}", before.elapsed(), max_calories);
}

pub fn part_two() {
    let before = std::time::Instant::now();

    let elves: Vec<Vec<i32>> = parse_input();

    let sums: Vec<i32> = elves.iter().map(|elf| elf.iter().sum()).collect();

    let sorted: Vec<i32> = sorted(sums.iter().copied()).rev().collect();

    let result: i32 = sorted.iter().take(3).sum();

    println!("[{:.2?}]\t Day 1 part 2 result: {}", before.elapsed(), result);
}