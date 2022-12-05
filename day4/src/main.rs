use std::ops::RangeInclusive;

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn main() {
    let input = load_input();

    let setup_time = std::time::Instant::now();
    let part1 = part1(&input);
    let part1_dur = setup_time.elapsed().as_micros();
    println!("Part1 : {} in {} µs", part1, part1_dur);

    let setup_time = std::time::Instant::now();
    let part2 = part2(&input);
    let part2_dur = setup_time.elapsed().as_micros();
    println!("Part2 : {} in {} µs", part2, part2_dur);
}

fn part1(input: &str) -> usize {
    solve(input, contains)
}

fn part2(input: &str) -> usize {
    solve(input, overlaps)
}

fn solve(
    input: &str,
    filter: impl Fn(&RangeInclusive<usize>, &RangeInclusive<usize>) -> bool,
) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut ranges = line.split(',');
            (
                make_range(ranges.next().unwrap()),
                make_range(ranges.next().unwrap()),
            )
        })
        .filter(|(r1, r2)| filter(r1, r2))
        .count()
}

fn make_range(txt: &str) -> RangeInclusive<usize> {
    let mut bounds = txt.split('-');
    bounds.next().unwrap().parse::<usize>().unwrap()
        ..=(bounds.next().unwrap().parse::<usize>().unwrap())
}

fn contains(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    (r1.start() <= r2.start() && r1.end() >= r2.end())
        || (r2.start() <= r1.start() && r2.end() >= r1.end())
}

fn overlaps(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    (r1.start() <= r2.start() && r1.end() >= r2.start())
        || (r2.start() <= r1.start() && r2.end() >= r1.start())
}