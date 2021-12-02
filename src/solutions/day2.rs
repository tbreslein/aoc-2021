trait SubData {
    fn new() -> Self;
    fn forward(&mut self, input: i32) -> ();
    fn up(&mut self, input: i32) -> ();
    fn down(&mut self, input: i32) -> ();
    fn mult(&self) -> i32;
}

struct Position {
    x: i32,
    depth: i32,
}

impl SubData for Position {
    fn new() -> Position {
        Position { x: 0, depth: 0 }
    }

    fn forward(&mut self, input: i32) {
        self.x += input
    }
    fn up(&mut self, input: i32) {
        self.depth -= input
    }
    fn down(&mut self, input: i32) {
        self.depth += input
    }
    fn mult(&self) -> i32 {
        self.x * self.depth
    }
}

struct PositionAndAim {
    pos: Position,
    aim: i32,
}

impl SubData for PositionAndAim {
    fn new() -> PositionAndAim {
        PositionAndAim {
            pos: Position::new(),
            aim: 0,
        }
    }

    fn forward(&mut self, input: i32) {
        self.pos.x += input;
        self.pos.depth += self.aim * input;
    }
    fn up(&mut self, input: i32) {
        self.aim -= input
    }
    fn down(&mut self, input: i32) {
        self.aim += input
    }
    fn mult(&self) -> i32 {
        self.pos.x * self.pos.depth
    }
}

fn solve<T: SubData>(data: &str) -> i32 {
    data.lines()
        .fold(T::new(), |mut acc, input| {
            let mut iter = input.split(" ");
            match (
                iter.next().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
            ) {
                ("forward", x) => acc.forward(x),
                ("up", y) => acc.up(y),     // we are in a submarine...
                ("down", y) => acc.down(y), // so up and down are weird
                _ => panic!("Parsing error!"),
            };
            acc
        })
        .mult()
}

pub fn solve_p1(data: &str) -> i32 {
    solve::<Position>(data)
}

pub fn solve_p2(data: &str) -> i32 {
    solve::<PositionAndAim>(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let data = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
        assert_eq!(solve_p1(data), 150);
    }
    #[test]
    fn test_p2() {
        let data = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
        assert_eq!(solve_p2(data), 900);
    }
}
