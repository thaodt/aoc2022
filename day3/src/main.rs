#![feature(iter_array_chunks)]

use std::{collections::HashSet, fmt::Display};

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn main() {
    let input = load_input();
    println!("Problem one: {}", part1(&input));
    println!("Problem two: {}", part2(&input));
}

fn map_string(i: &str) -> HashSet<u8> {
	i.bytes()
		.map(|b| match b {
			b'a'..=b'z' => b - b'a' + 1,
			b'A'..=b'Z' => b - b'A' + 27,
			_ => panic!(),
		})
		.collect()
}

pub fn part1(i: &str) -> impl Display {
	i.split('\n')
		.map(|line| {
			let (a, b) = line.split_at(line.len() / 2);
			let (a, b) = (map_string(a), map_string(b));
			let common = &a & &b;

			let v = common.into_iter().next();
            if v.is_some() {
                v.unwrap() as i64
            } else {
                0
            }
		})
		.sum::<i64>()
}

pub fn part2(i: &str) -> impl Display {
	i.split('\n')
		.array_chunks::<3>()
		.map(|group| group.map(map_string))
		.map(|[a, b, c]| {
			let common = &(&a & &b) & &c;
			common.into_iter().next().unwrap() as i64
		})
		.sum::<i64>()
}