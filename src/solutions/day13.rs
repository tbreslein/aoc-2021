use std::cmp;
use std::ops::Add;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Point {
    Dot,
    Blank,
    Empty,
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self {
            Point::Dot => match other {
                Point::Dot => Point::Dot,
                Point::Blank => Point::Dot,
                Point::Empty => Point::Empty,
            },
            Point::Blank => match other {
                Point::Dot => Point::Dot,
                Point::Blank => Point::Blank,
                Point::Empty => Point::Empty,
            },
            Point::Empty => match other {
                Point::Dot => Point::Dot,
                Point::Blank => Point::Blank,
                Point::Empty => Point::Empty,
            },
        }
    }
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

type Board = Vec<Vec<Point>>;

pub fn solve_p1(data: &str) -> usize {
    let (coords, folds) = parse_input(data);
    let (max_x, max_y) = find_max_coords(&coords);
    let mut board: Board = vec![vec![Point::Blank; max_x + 1]; max_y + 1];
    apply_marks(&mut board, &coords);
    apply_fold(&mut board, &folds[0], max_x, max_y);
    return count_dots(&board);
}

pub fn solve_p2(data: &str) -> String {
    let (coords, folds) = parse_input(data);
    let (max_x, max_y) = find_max_coords(&coords);
    let mut max_coords = Coord { x: max_x, y: max_y };
    let mut board: Board = vec![vec![Point::Blank; max_x + 1]; max_y + 1];
    apply_marks(&mut board, &coords);
    for fold in folds {
        max_coords = apply_fold(&mut board, &fold, max_coords.x, max_coords.y);
    }
    let mut output: String = "".to_string();
    for line in &board {
        if line.iter().all(|x| *x == Point::Empty) {
            continue;
        }
        for point in line {
            if let Point::Empty = point {
                continue;
            } else {
                if let Point::Dot = point {
                    output.push('#');
                } else {
                    output.push('.');
                }
            }
        }
        output.push('\n');
    }
    println!("output = {:?}", output);
    return output;
}

fn parse_input(data: &str) -> (Vec<Coord>, Vec<Fold>) {
    let tuple = data.split_once("\n\n");
    let coords: Vec<Coord> = tuple
        .unwrap()
        .0
        .lines()
        .map(|xy| {
            let coordtuple = xy.split_once(',').unwrap();
            return Coord {
                x: coordtuple.0.parse().unwrap(),
                y: coordtuple.1.parse().unwrap(),
            };
        })
        .collect();
    let folds: Vec<Fold> = tuple
        .unwrap()
        .1
        .lines()
        .map(|pos| {
            let fold_pos = pos
                .split_once(" along ")
                .unwrap()
                .1
                .split_once('=')
                .unwrap();
            return match fold_pos.0 {
                "x" => Fold::X(fold_pos.1.parse().unwrap()),
                "y" => Fold::Y(fold_pos.1.parse().unwrap()),
                _ => panic!("I dunno what happened"),
            };
        })
        .collect();
    return (coords, folds);
}

fn find_max_coords(coords: &Vec<Coord>) -> (usize, usize) {
    let (mut max_x, mut max_y) = (0, 0);
    for coord_pair in coords {
        max_x = cmp::max(max_x, coord_pair.x);
        max_y = cmp::max(max_y, coord_pair.y);
    }
    return (max_x, max_y);
}

fn apply_marks(board: &mut Board, coords: &Vec<Coord>) -> () {
    for coord in coords {
        (*board)[coord.y][coord.x] = Point::Dot;
    }
}

fn apply_fold(board: &mut Board, fold: &Fold, max_x: usize, max_y: usize) -> Coord {
    match fold {
        Fold::X(x) => {
            for i in 0..*x {
                for j in 0..=max_y {
                    (*board)[j][i] = board[j][i] + board[j][max_x - i];
                    (*board)[j][max_x - i] = Point::Empty;
                }
            }
            for i in *x..board[0].len() {
                for j in 0..=max_y {
                    (*board)[j][i] = Point::Empty;
                }
            }
            return Coord {
                x: (0..*x).count() - 1,
                y: max_y,
            };
        }
        Fold::Y(y) => {
            for j in 0..*y {
                for i in 0..=max_x {
                    (*board)[j][i] = board[j][i] + board[max_y - j][i];
                }
            }
            for j in *y..board.len() {
                for i in 0..=max_x {
                    (*board)[j][i] = Point::Empty;
                }
            }
            return Coord {
                x: max_x,
                y: (0..*y).count() - 1,
            };
        }
    };
}

fn count_dots(board: &Board) -> usize {
    board
        .into_iter()
        .flatten()
        .filter(|x| *x == &Point::Dot)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_p1() {
        let data = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
        assert_eq!(solve_p1(data), 17);
    }

    #[test]
    fn test_p2_1() {
        let data = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
        let should_be = r"#####
#...#
#...#
#...#
#####
.....
.....
";
        assert_eq!(solve_p2(data), should_be);
    }

    #[test]
    fn test_p2_2() {
        let data = fs::read_to_string("data/day13.txt").unwrap();
        let should_be = r".##..###..#..#.####.###...##..#..#.#..#.
#..#.#..#.#..#....#.#..#.#..#.#..#.#..#.
#..#.#..#.####...#..#..#.#....#..#.####.
####.###..#..#..#...###..#....#..#.#..#.
#..#.#.#..#..#.#....#....#..#.#..#.#..#.
#..#.#..#.#..#.####.#.....##...##..#..#.
";
        assert_eq!(solve_p2(&data), should_be);
    }
}
