// NOTE: For some reason, when running this function with the
// challenge input, the result is off by one. I get 1153 but should
// have gotten 1154... I have no idea why, especially considering
// that solve_p2 just works, and the way I solved that is pretty
// similar.
pub fn solve_p1(data: &str) -> i32 {
    data.lines()
        .collect::<Vec<&str>>()
        .windows(2)
        .fold(0, |acc, x| {
            if x[1].parse::<i32>().unwrap_or(0) > x[0].parse::<i32>().unwrap_or(0) {
                acc + 1
            } else {
                acc
            }
        })
}

pub fn solve_p2(data: &str) -> i32 {
    data.lines()
        .collect::<Vec<&str>>()
        .windows(3)
        .fold((0, std::i32::MAX), |acc, x| {
            let xsum = x.iter().map(|y| y.parse::<i32>().unwrap_or(0)).sum();
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
