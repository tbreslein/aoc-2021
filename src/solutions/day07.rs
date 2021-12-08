use std::cmp;

fn parse_input(data: &str) -> Vec<i32> {
    data.split_once("\n")
        .unwrap()
        .0
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn calc_fuel_p1(crabs: &Vec<i32>, i: i32) -> i32 {
    crabs
        .iter()
        .fold(0, |fuel, crab_position| fuel + (crab_position - i).abs())
}

fn calc_fuel_p2(crabs: &Vec<i32>, i: i32) -> i32 {
    crabs.iter().fold(0, |fuel, crab_position| {
        fuel + (1..=(crab_position - i).abs()).sum::<i32>()
    })
}

fn solve(data: &str, calc_fuel: fn(&Vec<i32>, i32) -> i32) -> i32 {
    let crabs = parse_input(data);
    (*crabs.iter().min().unwrap()..*crabs.iter().max().unwrap()).fold(i32::MAX, |current_min, i| {
        cmp::min(current_min, calc_fuel(&crabs, i))
    })
}

pub fn solve_p1(data: &str) -> i32 {
    solve(data, calc_fuel_p1)
}

pub fn solve_p2(data: &str) -> i32 {
    solve(data, calc_fuel_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let data = "16,1,2,0,4,2,7,1,2,14\n";
        assert_eq!(solve_p1(data), 37);
    }

    #[test]
    fn test_p2() {
        let data = "16,1,2,0,4,2,7,1,2,14\n";
        assert_eq!(solve_p2(data), 168);
    }
}
