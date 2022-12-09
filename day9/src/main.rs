use std::collections::HashSet;

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn main() {
    let input = load_input();
    // println!("{}", input);

    let mut second_visited = HashSet::new();
    let mut tail_visited = HashSet::new();

    let mut positions = [(0_isize, 0_isize); 10];
    for line in input.lines() {
        let mut iter = line.split(' ');

        let step_to_take = match iter.next().unwrap() {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!(),
        };

        for _ in 0..iter.next().unwrap().parse::<usize>().unwrap() {
            let head = &mut positions[0];
            head.0 += step_to_take.0;
            head.1 += step_to_take.1;

            for i in 0..9 {
                let x = (positions[i].0 - positions[i + 1].0).signum();
                let y = (positions[i].1 - positions[i + 1].1).signum();
                if x * (positions[i].0 - positions[i + 1].0) > 1
                    || y * (positions[i].1 - positions[i + 1].1) > 1
                {
                    positions[i + 1] = (positions[i + 1].0 + x, positions[i + 1].1 + y);
                }
            }

            second_visited.insert(positions[1]);
            tail_visited.insert(positions[9]);
        }
    }

    let p1 = second_visited.len();
    let p2 = tail_visited.len();

    println!("part 1: {p1}");
    println!("part 2: {p2}");
}
