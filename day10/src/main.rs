fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn main() {
    let input = load_input();
    
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}

fn p1(input: &str) -> i32 {
    let mut it = input.lines();
    let mut x = 1;
    let mut sum = 0;
    let mut add: Option<i32> = None;
    for cycle in 1i32.. {
        //if cycle > 180 { println!("{sum} {cycle} {x} {add:?}"); }
        if (cycle - 20) % 40 == 0 {
            //dbg!((sum, cycle, x, x * cycle));
            sum += cycle * x;
            if cycle == 220 {
                break;
            }
        }
        if let Some(add) = add.take() {
            x += add;
            continue;
        }

        let s = it.next();
        //if cycle > 180 { println!("{s:?}"); }
        match s {
            Some(s) => match s {
                "noop" => (),
                addx => {
                    let v = addx[addx.find(" ").expect("no space") + 1..]
                        .parse()
                        .expect("not a number");
                    add = Some(v);
                }
            },
            None => break,
        }
    }
    sum
}

fn p2(input: &str) -> String {
    let mut it = input.lines();
    let mut x = 1;
    let mut output = Vec::with_capacity(4096);
    let mut add: Option<i32> = None;
    for cycle in 1i32.. {
        let beam = (cycle - 1) % 40;
        if (beam - x).abs() <= 1 {
            output.push('#');
        } else {
            output.push('.');
        }
        if let Some(add) = add.take() {
            x += add;
            continue;
        }

        match it.next() {
            Some(s) => match s {
                "noop" => (),
                addx => {
                    let v = addx[addx.find(" ").expect("no space") + 1..]
                        .parse()
                        .expect("not a number");
                    add = Some(v);
                }
            },
            None => break,
        }
    }
    output.truncate(output.len() - 1);
    format!(
        "\n{}",
        output
            .chunks(40)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    )
}