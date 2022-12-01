macro_rules! loaders {
    ($($fn: ident => $file: expr$(,)*)*) => {
	$(
	    pub fn $fn() -> String {
		std::fs::read_to_string($file).unwrap()
	    }
	)*
    }
}

loaders! {
    load_input => "input",
}

fn p1(inp: &str) -> Vec<usize> {
    let mut elves = Vec::new();
    let mut sum = 0;
    for line in inp.lines() {
        if line.is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<usize>().unwrap();
        }
    }
    elves.push(sum);
    elves
}

fn main() {
    let input = load_input();
    let mut elves = p1(&input);
    elves.sort();
    elves.reverse();
    println!("part1 = {}", elves.iter().max().unwrap());
    println!("part2 = {}", elves[..3].iter().sum::<usize>());
}