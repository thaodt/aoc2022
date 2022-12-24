use helpers::*;
use std::collections::{HashMap, HashSet};

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn main() {
    let input = load_input();
    println!("part 1 {}", part1(&input));
    println!("part 2 {}", part2(&input));
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let grid = input.lines().map(|l| l.bytes().collectvec()).collectvec();
    let w = grid[0].len() as i64;
    let h = grid.len() as i64;
    let mut bliz: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            match grid[y as usize][x as usize] {
                b'<' => bliz.entry((x, y)).or_default().push((-1, 0)),
                b'>' => bliz.entry((x, y)).or_default().push((1, 0)),
                b'v' => bliz.entry((x, y)).or_default().push((0, 1)),
                b'^' => bliz.entry((x, y)).or_default().push((0, -1)),
                _ => (),
            }
        }
    }
    for i in 0..h {
        bliz.insert((0, i), vec![(0, 0)]);
        bliz.insert((w - 1, i), vec![(0, 0)]);
    }
    for i in 0..w {
        bliz.insert((i + 2, 0), vec![(0, 0)]);
        bliz.insert((i - 2, h - 1), vec![(0, 0)]);
    }
    bliz.insert((1, -1), vec![(0, 0)]);
    bliz.insert((w - 2, h), vec![(0, 0)]);
    let mut q = HashSet::new();
    let mut qq = HashSet::new();
    let mut bliz2: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    q.insert((1, 0));
    let mut minute = 0;
    loop {
        minute += 1;
        for (p, b) in bliz.drain() {
            for (dx, dy) in b {
                let mut pp = (p.0 + dx, p.1 + dy);
                if dx != 0 || dy != 0 {
                    if pp.0 == 0 {
                        pp.0 = w - 2;
                    }
                    if pp.0 == w - 1 {
                        pp.0 = 1;
                    }
                    if pp.1 == 0 {
                        pp.1 = h - 2;
                    }
                    if pp.1 == h - 1 {
                        pp.1 = 1;
                    }
                }
                bliz2.entry(pp).or_default().push((dx, dy));
            }
        }
        for (x, y) in q.drain() {
            if (x, y) == (w - 2, h - 1) {
                return minute - 1;
            }
            for c in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1), (x, y)] {
                if !bliz2.contains_key(&c) {
                    qq.insert(c);
                }
            }
        }

        std::mem::swap(&mut q, &mut qq);
        std::mem::swap(&mut bliz, &mut bliz2);
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let grid = input.lines().map(|l| l.bytes().collectvec()).collectvec();
    let w = grid[0].len() as i64;
    let h = grid.len() as i64;
    let mut bliz: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            match grid[y as usize][x as usize] {
                b'<' => bliz.entry((x, y)).or_default().push((-1, 0)),
                b'>' => bliz.entry((x, y)).or_default().push((1, 0)),
                b'v' => bliz.entry((x, y)).or_default().push((0, 1)),
                b'^' => bliz.entry((x, y)).or_default().push((0, -1)),
                _ => (),
            }
        }
    }
    for i in 0..h {
        bliz.insert((0, i), vec![(0, 0)]);
        bliz.insert((w - 1, i), vec![(0, 0)]);
    }
    for i in 0..w {
        bliz.insert((i + 2, 0), vec![(0, 0)]);
        bliz.insert((i - 2, h - 1), vec![(0, 0)]);
    }
    bliz.insert((1, -1), vec![(0, 0)]);
    bliz.insert((w - 2, h), vec![(0, 0)]);
    let mut q = HashSet::new();
    let mut qq = HashSet::new();
    let mut bliz2: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    q.insert((1, 0));
    let mut minute = 0;
    let mut phase = 0;
    loop {
        minute += 1;
        for (p, b) in bliz.drain() {
            for (dx, dy) in b {
                let mut pp = (p.0 + dx, p.1 + dy);
                if dx != 0 || dy != 0 {
                    if pp.0 == 0 {
                        pp.0 = w - 2;
                    }
                    if pp.0 == w - 1 {
                        pp.0 = 1;
                    }
                    if pp.1 == 0 {
                        pp.1 = h - 2;
                    }
                    if pp.1 == h - 1 {
                        pp.1 = 1;
                    }
                }
                bliz2.entry(pp).or_default().push((dx, dy));
            }
        }
        for (x, y) in q.drain() {
            if phase == 0 && (x, y) == (w - 2, h - 1) || phase == 1 && (x, y) == (1, 0) {
                phase += 1;
                qq.clear();
                qq.insert((x, y));
                break;
            } else if phase == 2 && (x, y) == (w - 2, h - 1) {
                return minute - 1;
            }
            for c in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1), (x, y)] {
                if !bliz2.contains_key(&c) {
                    qq.insert(c);
                }
            }
        }

        std::mem::swap(&mut q, &mut qq);
        std::mem::swap(&mut bliz, &mut bliz2);
    }
}
