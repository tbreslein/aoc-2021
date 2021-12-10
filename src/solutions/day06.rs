pub fn solve_p1(data: &str) -> u64 {
    solve(data, 80)
}

pub fn solve_p2(data: &str) -> u64 {
    solve(data, 256)
}

fn parse_input(data: &str) -> [u64; 9] {
    data.split_once("\n")
        .unwrap()
        .0
        .split(",")
        .map(|x| x.parse().unwrap())
        .fold([0_u64; 9], |mut fish_timers, val: usize| {
            fish_timers[val] += 1;
            fish_timers
        })
}

fn solve(data: &str, days: i32) -> u64 {
    (1..=days)
        .fold(parse_input(data), |mut fish_timers, _| {
            // number of fish that spawn new fish
            let fishies_at_zero = fish_timers[0];
            // age every other fish
            for i in 0..=7 {
                fish_timers[i] = fish_timers[i + 1];
            }
            // spawn new fish
            fish_timers[8] = fishies_at_zero;
            // cycle the fish that spawned new fish
            fish_timers[6] += fishies_at_zero;
            fish_timers
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let data = "3,4,3,1,2\n";

        assert_eq!(solve_p1(data), 5934);
    }

    #[test]
    fn test_p2() {
        let data = "3,4,3,1,2\n";
        assert_eq!(solve_p2(data), 26984457539);
    }
}
