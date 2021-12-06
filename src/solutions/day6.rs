use std::cmp;

fn parse_input(data: &str) -> Vec<i32> {
    data.split_once("\n")
        .unwrap()
        .0
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn precompute(fishy_timer: i32, days: i32) -> u64 {
    // println!("starting precompute for fishy_timer = {:?}", fishy_timer);
    // (1..=days)
    //     .fold(vec![fishy_timer; 1], |mut school, day| {
    //         println!("initial_timer = {:?}; day = {:?}", fishy_timer, day);
    //         let mut number_of_new_fishies: usize = 0;
    //         for fishy in school.iter_mut() {
    //             if *fishy <= 0 {
    //                 number_of_new_fishies += 1;
    //                 *fishy = 6;
    //             } else {
    //                 *fishy -= 1;
    //             }
    //         }
    //         school.append(&mut vec![8; number_of_new_fishies]);
    //         school
    //     })
    //     .len() as u64
    let dprime = days + (8 - fishy_timer);
    let new_fishies =
        (cmp::max(0, dprime - 8) + cmp::max(0, dprime - 8) / 7).pow((dprime / 8) as u32);
    new_fishies as u64
}

// The trick here is that brute forcing this problem is very computionally
// expensive. But, fortunately, this problem can easily be solved analytically.
//
// Let's assume we only have one fish for now.
// To get at the formula, we approach this step by step:
// - calculate how many children the initial fish spawns in the time span
// - recursively, calculate how many children each those fish spawn
//
// Let's declare some variables:
// - t*: initial fish timer
// - d*: initlal number of days
// We can hide the init_fish_timer, by fudging the initial condition a bit.
// Instead of starting with a fish at a timer of t for d* days, assuming that
// t < d*, just start right away with:
// t = 8
// d = d* + (8 - t*)
//
// With that, we can argue that any "initial fish problem" kind of always
// starts with the fish timer at 8. This means that the initial fish isn't
// actually different from the other new fishies.
//
// With that, any fresh fish's spawn count can be calculated with:
// - make sure that d' = d - 1 > 7
// - if so, this fish spawns 1 + (d' - 8) // 7
//
// or as pseudo code with d' being the remaining days after spawning this fish:
// new_fishies = max(0, d' - 8) + (d' - 8) // 7 : 0
//
// Now we get to recurse. Each of those new fishies will have d'' = d'-8 days
// left, so they each will spawn d'' > 7 ? 1 + (d'' - 8) // 7 : 0 new fishies.
//
// This where we get the exponential growth, because as long as we have more
// than 8 days left, we get another recursion, so the full number of recursion
// cycles is d // 8.
fn solve(data: &str, days: i32) -> u64 {
    let precomputations = [
        precompute(1, days),
        precompute(2, days),
        precompute(3, days),
        precompute(4, days),
        precompute(5, days),
    ];
    parse_input(data)
        .iter()
        .fold(0_u64, |final_fishy_count, init_fishy| {
            final_fishy_count + precomputations[(*init_fishy as i32 - 1) as usize]
        })
    // 0
}

pub fn solve_p1(data: &str) -> u64 {
    solve(data, 80)
}

pub fn solve_p2(_data: &str) -> u64 {
    // solve(data, 256)
    0
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
