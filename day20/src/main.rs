const KEY: i64 = 811589153;

type Number = i64;
type LinkId = usize;
type NumberId = usize;

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn main() {
    let input = load_input();
    println!("part 1 {}", part1(&input));
    println!("part 2 {}", part2(&input));
}

struct State {
    numbers: Vec<Number>,
    links: Vec<Link>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Link {
    left: LinkId,
    right: LinkId,
    number: Number,
    index: LinkId,
}

impl State {
    fn new(numbers: Vec<Number>) -> Self {
        let mut links = vec![];
        for i in 0..numbers.len() {
            let link = Link {
                left: pos_mod(i as Number - 1, numbers.len()),
                right: (i + 1) % numbers.len(),
                number: numbers[i],
                index: i,
            };
            links.push(link);
        }
        Self { numbers, links }
    }

    fn parse(input: &str) -> Self {
        let numbers = input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        Self::new(numbers)
    }

    fn move_number(&mut self, num_index: NumberId, key: Number) {
        let link = &self.links[num_index];
        let number = link.number * key;
        let moves = number % (self.links.len() as Number - 1);
        let direction = moves.signum();
        for _ in 0..moves.abs() {
            self.step(num_index, direction);
        }
    }

    fn remove_link(&mut self, index: NumberId) {
        let (left, right) = {
            let link = &self.links[index];
            (link.left, link.right)
        };
        self.links[left].right = right;
        self.links[right].left = left;
    }

    fn insert_link(&mut self, index: NumberId, left: NumberId, right: NumberId) {
        self.links[left].right = index;
        self.links[right].left = index;
        let link = &mut self.links[index];
        link.left = left;
        link.right = right;
    }

    fn step(&mut self, index: NumberId, direction: Number) {
        let link = self.links[index];
        if direction > 0 {
            self.remove_link(index);
            self.insert_link(index, link.right, self.links[link.right].right);
        } else {
            self.remove_link(index);
            self.insert_link(index, self.links[link.left].left, link.left);
        }
    }

    fn move_right(&self, index: LinkId) -> LinkId {
        self.links[index].right
    }

    fn index_of(&self, number: Number) -> LinkId {
        let mut index = 0;
        while self.links[index].number != number {
            index = self.move_right(index);
        }
        index
    }

    fn mix(&mut self, iterations: usize, key: Number) -> [LinkId; 3] {
        for _ in 0..iterations {
            for i in 0..self.numbers.len() {
                self.move_number(i, key);
            }
        }
        let mut result = [0; 3];
        let zero_index = self.index_of(0);
        let mut index = zero_index;
        for i in 0..3 {
            for _ in 0..1000 {
                index = self.move_right(index);
            }
            result[i] = index;
        }
        result
    }
}

pub fn part1(input: &str) -> i64 {
    let mut state = State::parse(input);
    state
        .mix(1, 1)
        .iter()
        .map(|&index| state.links[index].number)
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let mut state = State::parse(input);
    state
        .mix(10, KEY)
        .iter()
        .map(|&index| state.links[index].number * KEY)
        .sum()
}

fn pos_mod(val: i64, div: usize) -> usize {
    let div = div as i64;
    ((val % div + div) % div) as usize
}
