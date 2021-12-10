use std::cmp;

struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(input: &str) -> Coord {
        let tuple = input.split_once(",").unwrap();
        Coord {
            x: tuple.0.parse().unwrap_or(0),
            y: tuple.1.parse().unwrap_or(0),
        }
    }
}

struct Line {
    begin: Coord,
    end: Coord,
}

impl Line {
    fn new(input: &str) -> Line {
        let tuple = input.split_once(" -> ").unwrap();
        Line {
            begin: Coord::new(tuple.0),
            end: Coord::new(tuple.1),
        }
    }

    fn is_diagonal(&self) -> bool {
        self.begin.x != self.end.x && self.begin.y != self.end.y
    }

    fn get_length(&self) -> i32 {
        if self.begin.x == self.end.x {
            // cannot use x for the line length in this case
            (self.begin.y - self.end.y).abs()
        } else {
            (self.begin.x - self.end.x).abs()
        }
    }

    // not a particularly fitting name, to be fair...
    fn get_y_index(&self, i: i32) -> usize {
        (if self.begin.y == self.end.y {
            self.begin.y
        } else if self.begin.y < self.end.y {
            self.begin.y + i
        } else {
            self.begin.y - i
        }) as usize
    }

    fn get_x_index(&self, i: i32) -> usize {
        (if self.begin.x == self.end.x {
            self.begin.x
        } else if self.begin.x < self.end.x {
            self.begin.x + i
        } else {
            self.begin.x - i
        }) as usize
    }
}

type Lines = Vec<Line>;
type Board = Vec<Vec<i32>>;

pub fn solve_p1(data: &str) -> usize {
    let (mut board, lines) = parse_input(data);
    for line in lines.iter() {
        if line.is_diagonal() {
            continue;
        }
        apply_line_to_board(&line, &mut board);
    }
    count_overlaps(&board)
}

pub fn solve_p2(data: &str) -> usize {
    let (mut board, lines) = parse_input(data);
    for line in lines.iter() {
        apply_line_to_board(&line, &mut board);
    }
    count_overlaps(&board)
}

fn parse_input(data: &str) -> (Board, Lines) {
    let lines = data.lines().map(|x| Line::new(x)).collect();
    let board_size = determine_board_size(&lines);
    let board = vec![vec![0; board_size.0 + 1]; board_size.1 + 1];
    (board, lines)
}

fn determine_board_size(lines: &Lines) -> (usize, usize) {
    let mut biggest_x = 0;
    let mut biggest_y = 0;
    for line in lines.iter() {
        biggest_x = cmp::max(biggest_x, cmp::max(line.begin.x, line.end.x));
        biggest_y = cmp::max(biggest_y, cmp::max(line.begin.y, line.end.y));
    }
    (biggest_x as usize, biggest_y as usize)
}

fn apply_line_to_board(line: &Line, board: &mut Board) {
    for i in 0..=line.get_length() {
        board[line.get_y_index(i)][line.get_x_index(i)] += 1;
    }
}

fn count_overlaps(board: &Board) -> usize {
    board.iter().flatten().filter(|x| x > &&1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let data = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

        assert_eq!(solve_p1(data), 5);
    }

    #[test]
    fn test_p2() {
        let data = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

        assert_eq!(solve_p2(data), 12);
    }
}
