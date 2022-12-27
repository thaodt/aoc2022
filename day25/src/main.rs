fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn main() {
    let input = load_input();
    println!("part 1 {}", part1(&input));
    println!("part 2 {}", part2(&input));
}

pub fn part1(input: &str) -> String {
    let input: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let mut n = input
        .iter()
        .map(|&line| {
            let mut acc = 0i64;
            for &b in line {
                acc = acc * 5
                    + match b {
                        b'0' => 0,
                        b'1' => 1,
                        b'2' => 2,
                        b'-' => -1,
                        b'=' => -2,
                        _ => panic!("Invalid input"),
                    }
            }
            acc as u64
        })
        .sum::<u64>();
    let mut out: Vec<char> = Vec::new();
    while n != 0 {
        let (carry, character) = match n % 5 {
            0 => (0, '0'),
            1 => (0, '1'),
            2 => (0, '2'),
            3 => (1, '='),
            4 => (1, '-'),
            _ => unreachable!(),
        };
        n = n / 5 + carry;
        out.push(character);
    }
    out.into_iter().rev().collect()
}

pub fn part2(_input: &str) -> impl std::fmt::Display {
    0
}
