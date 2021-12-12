#[derive(Debug)]
enum State {
    Normal(u32),
    Flashing,
    Exhausted,
}

type Board = Vec<Vec<State>>;

pub fn solve_p1(data: &str) -> u32 {
    let mut flashes = 0;
    let mut board = parse_input(data);
    let board_length = board.len();

    // the time steps
    for _ in 1..=100 {
        // step 1: increment every energy level
        step1(&mut board, board_length);

        // step 2: flashing
        step2(&mut board, board_length);

        // step 3: reset flashing octopi to 0 and increment flashes
        for j in 0..board_length {
            for i in 0..board_length {
                if let State::Exhausted = board[j][i] {
                    board[j][i] = State::Normal(0);
                    flashes += 1;
                }
            }
        }
    }

    return flashes;
}

pub fn solve_p2(data: &str) -> u64 {
    let mut board = parse_input(data);
    let board_length = board.len();

    // the time steps
    for t in 1.. {
        // step 1: increment every energy level
        step1(&mut board, board_length);

        // step 2: flashing
        step2(&mut board, board_length);

        // intermission: return if all are exhausted
        if board.iter().flatten().all(|x| {
            if let State::Exhausted = x {
                true
            } else {
                false
            }
        }) {
            return t;
        }

        // step 3: reset flashing octopi to 0
        for j in 0..board_length {
            for i in 0..board_length {
                if let State::Exhausted = board[j][i] {
                    board[j][i] = State::Normal(0);
                }
            }
        }
    }

    return 1;
}

fn parse_input(data: &str) -> Vec<Vec<State>> {
    data.lines()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| State::Normal(c.to_digit(10).unwrap()))
                .collect()
        })
        .collect()
}

fn step1(board: &mut Board, board_length: usize) {
    for j in 0..board_length {
        for i in 0..board_length {
            match board[j][i] {
                State::Normal(x) => {
                    board[j][i] = if x >= 9 {
                        State::Flashing
                    } else {
                        State::Normal(x + 1)
                    }
                }
                State::Flashing => {
                    panic!("Error: there shouldn't have been Flashing left after step 2")
                }
                State::Exhausted => {
                    panic!("Error: there shouldn't have been Exhausted left after step 3")
                }
            }
        }
    }
}

fn step2(board: &mut Board, board_length: usize) {
    while board
        .iter()
        .flatten()
        .any(|x| if let State::Flashing = x { true } else { false })
    {
        for j in 0..board_length {
            for i in 0..board_length {
                if let State::Flashing = board[j][i] {
                    light_it_up(board, j, i);
                }
            }
        }
    }
}

fn light_it_up(board: &mut Vec<Vec<State>>, j: usize, i: usize) {
    board[j][i] = State::Exhausted;
    let board_length = board.len();
    for j_step in -1..=1 {
        if (j_step == -1 && j == 0) || (j_step == 1 && j == board_length - 1) {
            continue;
        }
        for i_step in -1..=1 {
            if (i_step == -1 && i == 0) || (i_step == 1 && i == board_length - 1) {
                continue;
            }
            let j_new = (j as i32 + j_step) as usize;
            let i_new = (i as i32 + i_step) as usize;
            if let State::Normal(x) = board[j_new][i_new] {
                if x >= 9 {
                    board[j_new][i_new] = State::Flashing;
                    light_it_up(board, j_new, i_new);
                } else {
                    board[j_new][i_new] = State::Normal(x + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let data = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";
        assert_eq!(solve_p1(data), 1656);
    }

    #[test]
    fn test_p2() {
        let data = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";
        assert_eq!(solve_p2(data), 195);
    }
}
