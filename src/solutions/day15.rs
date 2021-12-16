use priority_queue::PriorityQueue;
use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    j: usize,
    i: usize,
}

impl Position {
    fn new(j: usize, i: usize) -> Self {
        Position { j, i }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Node {
    position: Position,
    local_risk: Risk,
    distance: Risk,
    visited: bool,
}

impl Node {
    fn new(j_in: usize, i_in: usize, local_risk: i32) -> Self {
        Node {
            position: Position::new(j_in, i_in),
            local_risk,
            distance: i32::MAX,
            visited: false,
        }
    }
}

type Risk = i32;
type Board = Vec<Vec<Node>>;

pub fn solve_p1(data: &str) -> i32 {
    return find_path(&mut parse_input(data));
}

pub fn solve_p2(_data: &str) -> i32 {
    1
}

fn parse_input(data: &str) -> Board {
    data.lines()
        .enumerate()
        .fold(Vec::new(), |mut outer, (j, line)| {
            outer.push(
                line.chars()
                    .enumerate()
                    .fold(Vec::new(), |mut inner, (i, c)| {
                        inner.push(Node::new(j, i, c.to_digit(10).unwrap() as i32));
                        return inner;
                    }),
            );
            return outer;
        })
}

fn find_path(board: &mut Board) -> i32 {
    board[0][0].local_risk = 0;
    board[0][0].distance = board[0][0].local_risk;

    let mut visited_nodes: HashSet<Position> = HashSet::new();
    let mut queue: PriorityQueue<Position, Risk> = PriorityQueue::new();
    queue.push(Position { j: 0, i: 0 }, 0);

    while let Some(node) = queue.pop() {
        if visited_nodes.contains(&Position {
            j: board.len() - 1,
            i: board[0].len() - 1,
        }) {
            break;
        }

        let j = node.0.j;
        let i = node.0.i;
        let this_distance = board[j][i].distance;
        println!("(j, i) = ({:?}, {:?})", j, i);
        println!("this_distance = {:?}", this_distance);

        if i < board[0].len() - 1 {
            discover(
                board,
                &mut queue,
                &mut visited_nodes,
                j,
                i + 1,
                this_distance,
            );
        }
        if j < board.len() - 1 {
            discover(
                board,
                &mut queue,
                &mut visited_nodes,
                j + 1,
                i,
                this_distance,
            );
        }
        if i > 0 {
            discover(
                board,
                &mut queue,
                &mut visited_nodes,
                j,
                i - 1,
                this_distance,
            );
        }
        if j > 0 {
            discover(
                board,
                &mut queue,
                &mut visited_nodes,
                j - 1,
                i,
                this_distance,
            );
        }

        visited_nodes.insert(board[j][i].position);
    }
    return board[board.len() - 1][board[0].len() - 1].distance;
}

fn discover(
    board: &mut Board,
    queue: &mut PriorityQueue<Position, Risk>,
    visited_nodes: &HashSet<Position>,
    j_new: usize,
    i_new: usize,
    last_distance: i32,
) {
    if !visited_nodes.contains(&board[j_new][i_new].position) {
        let new_distance = last_distance + board[j_new][i_new].local_risk;
        board[j_new][i_new].distance = new_distance;
        queue.push(board[j_new][i_new].position, -new_distance);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let data = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";
        assert_eq!(solve_p1(data), 40);
    }

    #[test]
    fn test_p2() {
        let data = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";
        assert_eq!(solve_p2(data), 0);
    }
}
