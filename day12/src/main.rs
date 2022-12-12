use helpers::*;
use std::collections::*;

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn main() {
    let input = load_input();
    println!("part 1 {}", part1(&input));
    println!("part 2 {}", part2(&input));
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    // a is low, z is high
    let mut grid = input.lines().map(|x| x.chars().collectvec()).collectvec();
    let w = grid[0].len();
    let h = grid.len();

    let mut s = (0, 0);
    let mut e = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 'S' {
                s = (i, j);
                grid[i][j] = 'a';
            }
            if grid[i][j] == 'E' {
                e = (i, j);
                grid[i][j] = 'z';
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(s);
    queue.push_back((s, 0));
    while let Some(((i, j), d)) = queue.pop_front() {
        if (i, j) == e {
            return d;
        }

        for di in [-1i32, 0, 1] {
            for dj in [-1i32, 0, 1] {
                if di == 0 || dj == 0 && !(di == 0 && dj == 0) {
                    let ii = (i as i32 + di) as usize;
                    let jj = (j as i32 + dj) as usize;
                    if ii >= h || jj >= w {
                        continue;
                    }
                    if grid[ii][jj] as i32 - grid[i][j] as i32 <= 1 {
                        if !visited.contains(&(ii, jj)) {
                            queue.push_back(((ii, jj), d + 1));
                            visited.insert((ii, jj));
                        }
                    }
                }
            }
        }
    }

    -1
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    // a is low, z is high
    let mut grid = input.lines().map(|x| x.chars().collectvec()).collectvec();
    let w = grid[0].len();
    let h = grid.len();

    let mut e = (0, 0);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 'S' || grid[i][j] == 'a' {
                queue.push_back(((i, j), 0));
                visited.insert((i, j));
                grid[i][j] = 'a';
            }
            if grid[i][j] == 'E' {
                e = (i, j);
                grid[i][j] = 'z';
            }
        }
    }

    while let Some(((i, j), d)) = queue.pop_front() {
        if (i, j) == e {
            return d;
        }

        for di in [-1i32, 0, 1] {
            for dj in [-1i32, 0, 1] {
                if di == 0 || dj == 0 && !(di == 0 && dj == 0) {
                    let ii = (i as i32 + di) as usize;
                    let jj = (j as i32 + dj) as usize;
                    if ii >= h || jj >= w {
                        continue;
                    }
                    if grid[ii][jj] as i32 - grid[i][j] as i32 <= 1 {
                        if !visited.contains(&(ii, jj)) {
                            queue.push_back(((ii, jj), d + 1));
                            visited.insert((ii, jj));
                        }
                    }
                }
            }
        }
    }

    -1
}
