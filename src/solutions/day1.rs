/// In Haskell this would be one liner like:
/// foldr (\(x,y) acc -> if y > x then acc + 1 else acc) 0 $ zip (lines xs) (drop 1 (lines xs))
///
/// Here, since we need something similar for p2, we use Rust's windows to get a sliding window
/// through the iterator that we read the values from.
///
/// Note that we don't need to parse the strings to i32, since we can compare the strings
/// lexicographically.
///
/// NOTE: For some reason, when running this function with the challenge input, the result is off
/// by one. I get 1153 but should have gotten 1154... I have no idea why, especially considering
/// that solve_p2 just works, and the way I solved that is pretty similar.
pub fn solve_p1(data: &str) -> i32 {
    data.lines()
        .collect::<Vec<&str>>()
        .windows(2)
        .fold(0, |acc, x| if x[1] > x[0] { acc + 1 } else { acc })
}

/// In Haskell, we could simply zip3, but here I feel like windows(3) is more ergonomic
pub fn solve_p2(data: &str) -> i32 {
    data.lines()
        .collect::<Vec<&str>>()
        .windows(3) // get a sliding window of that Vec<&str>
        // the accumulator is a tuple of ints, because I need to keep track of two things:
        // the current sum of how many increases I have, and what the sum in the last step was
        .fold((0, std::i32::MAX), |acc, x| {
            let xsum = x.iter().map(|y| y.parse::<i32>().unwrap()).sum();
            if xsum > acc.1 {
                (acc.0 + 1, xsum)
            } else {
                (acc.0, xsum)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let data = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
        assert_eq!(solve_p1(data), 7);
    }
    #[test]
    fn test_p2() {
        let data = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
        assert_eq!(solve_p2(data), 5);
    }
}
