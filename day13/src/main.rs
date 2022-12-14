use helpers::*;
use std::cmp::Ordering;

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn main() {
    let input = load_input();
    println!("part 1 {}", part1(&input));
    println!("part 2 {}", part2(&input));
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Value {
    Int(i64),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, r: &Value) -> Ordering {
        use Value::{Int, List};
        match (self, r) {
            (Int(a), Int(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Int(a), List(b)) => [Value::Int(*a)][..].cmp(b),
            (List(a), Int(b)) => (**a).cmp(&[Value::Int(*b)][..]),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(s: &mut &str) -> Value {
    if &s[..1] == "[" {
        let mut list = vec![];
        loop {
            *s = &s[1..];
            if &s[..1] == "]" {
                *s = &s[1..];
                break Value::List(list);
            }
            list.push(parse(s));
            if &s[..1] == "]" {
                *s = &s[1..];
                break Value::List(list);
            }
        }
    } else {
        let (a, _b) = s.split_once([',', ']']).unwrap();
        *s = &s[a.len()..];
        Value::Int(a.parse().unwrap())
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for (i, pair) in input.split("\n\n").enumerate() {
        let lines = pair.lines().collectvec();
        let first = lines[0];
        let second = lines[1];
        let first2 = parse(&mut { first });
        let second2 = parse(&mut { second });

        if first2 <= second2 {
            ans += i + 1;
        }
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let div1 = parse(&mut "[[2]]");
    let div2 = parse(&mut "[[6]]");
    let mut pkts = vec![div1.clone(), div2.clone()];
    for (_i, pair) in input.split("\n\n").enumerate() {
        let lines = pair.lines().collectvec();
        let first = lines[0];
        let second = lines[1];
        let first2 = parse(&mut { first });
        let second2 = parse(&mut { second });

        pkts.push(first2);
        pkts.push(second2);
    }
    pkts.sort_unstable();
    let a = pkts.iter().position(|v| v == &div1).unwrap();
    let b = pkts.iter().position(|v| v == &div2).unwrap();
    (a + 1) * (b + 1)
}
