use itertools::Itertools;

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn main() {
    let input = load_input();
    // println!("{}", input);
    //
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}

fn all_unique(chrs: Vec<char>) -> bool {
    chrs == chrs.clone().into_iter().unique().collect_vec()
}

pub fn p1(data: &str) -> String {
    let init = data.chars().collect_vec();
    let results = init
        .windows(4)
        .into_iter()
        .enumerate()
        .take_while(|(_, chars)| !all_unique(chars.to_vec()))
        .collect_vec();

    let (result, _) = results.last().unwrap();
    format!("{}", result + 5)
}

pub fn p2(data: &str) -> String {
    let init = data.chars().collect_vec();
    let results = init
        .windows(14)
        .into_iter()
        .enumerate()
        .take_while(|(_, chars)| !all_unique(chars.to_vec()))
        .collect_vec();

    let (result, _) = results.last().unwrap();
    format!("{}", result + 15)
}
