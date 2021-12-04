type Number = u32;
type Board = Vec<Vec<BoardNumber>>;
type DrawnNumbers = Vec<Number>;
type Boards = Vec<Board>;

#[derive(Debug, Clone, Copy)]
enum BoardNumber {
    UnMarked(Number),
    Marked(Number),
}

fn parse_input(data: &str) -> (DrawnNumbers, Boards) {
    let mut iter = data.split_at(data.len() - 1).0.split("\n\n"); // remove trailing \n
    let drawn_numbers: DrawnNumbers = iter
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<Number>().unwrap())
        .collect();
    let boards: Boards = iter
        .map(|block| {
            block
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .map(|line| {
                    line.trim()
                        .split_ascii_whitespace()
                        .map(|num| BoardNumber::UnMarked(num.parse::<Number>().unwrap()))
                        .collect()
                })
                .collect()
        })
        .collect();
    (drawn_numbers, boards)
}

fn mark_numbers_in_boards(drawn_number: Number, boards: &mut Boards) -> () {
    for board in boards.iter_mut() {
        for line in board.iter_mut() {
            for entry in line.iter_mut() {
                *entry = match *entry {
                    BoardNumber::UnMarked(x) => match x == drawn_number {
                        true => BoardNumber::Marked(x),
                        false => *entry,
                    },
                    _ => *entry,
                }
            }
        }
    }
}

fn sum_unmarked_on_board(board: &Board) -> u32 {
    board.iter().flatten().fold(0, |acc, y| {
        if let BoardNumber::UnMarked(num) = y {
            acc + num
        } else {
            acc + 0
        }
    })
}

fn check_if_any_board_won(boards: &Boards) -> Option<u32> {
    for board_i in 0..boards.len() {
        match check_if_specific_board_won(&boards[board_i]) {
            None => continue,
            Some(x) => return Some(x),
        };
    }
    None
}

fn check_if_specific_board_won(board: &Board) -> Option<u32> {
    // check rows
    for row in board.iter() {
        if row.iter().all(|x| match x {
            BoardNumber::Marked(_) => true,
            BoardNumber::UnMarked(_) => false,
        }) {
            return Some(sum_unmarked_on_board(&board));
        }
    }

    // check cols
    // need to go across rows now, so using an iterator
    // isn't as smooth anymore, so just use indeces
    // instead
    let num_cols = board[0].len();
    let num_rows = board.len();
    for col_i in 0..num_cols {
        let mut found_winning_col = true;
        for row_i in 0..num_rows {
            found_winning_col = found_winning_col
                && match board[row_i][col_i] {
                    BoardNumber::Marked(_) => true,
                    BoardNumber::UnMarked(_) => false,
                };
        }
        if found_winning_col {
            return Some(sum_unmarked_on_board(&board));
        }
    }
    None
}

pub fn solve_p1(data: &str) -> u32 {
    let (drawn_numbers, mut boards) = parse_input(data);
    for current_number in drawn_numbers {
        mark_numbers_in_boards(current_number, &mut boards);
        if let Some(x) = check_if_any_board_won(&boards) {
            return current_number * x;
        }
    }
    0
}

/*
 * Used by solve_p2 only
 */

fn remove_winning_boards(boards: &mut Boards) {
    while match check_if_any_board_won(&boards) {
        Some(_) => true,
        None => false,
    } {
        let mut maybe_remove_index = None;
        for (i, board) in boards.iter_mut().enumerate() {
            match check_if_specific_board_won(board) {
                None => continue,
                Some(_) => {
                    maybe_remove_index = Some(i);
                }
            };
        }
        if let Some(i) = maybe_remove_index {
            boards.remove(i);
        }
    }
}

pub fn solve_p2(data: &str) -> u32 {
    let (drawn_numbers, mut boards) = parse_input(data);
    for current_number in drawn_numbers {
        mark_numbers_in_boards(current_number, &mut boards);
        if boards.len() == 1 {
            if let Some(x) = check_if_specific_board_won(&boards[0]) {
                return current_number * x;
            }
        }
        remove_winning_boards(&mut boards);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let data1 = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

        let data2 = r"15,18,5,16,8,11,21

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

        assert_eq!(solve_p1(data1), 4512);
        assert_eq!(solve_p1(data2), 4830);
    }

    #[test]
    fn test_p2() {
        let data = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
        assert_eq!(solve_p2(data), 1924);
    }
}
