type HeightMap = Vec<Vec<u32>>;
type Coords = (usize, usize);

fn parse_input(data: &str) -> HeightMap {
    data.lines()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn find_low_points(map: &HeightMap) -> Vec<Coords> {
    map.into_iter()
        .enumerate()
        .fold(Vec::new(), |mut low_points, (j, row)| {
            row.into_iter().enumerate().for_each(|(i, current_point)| {
                if (i <= 0 || map[j][i - 1] > *current_point)
                    && (i >= row.len() - 1 || map[j][i + 1] > *current_point)
                    && (j <= 0 || map[j - 1][i] > *current_point)
                    && (j >= map.len() - 1 || map[j + 1][i] > *current_point)
                {
                    low_points.push((j, i));
                }
            });
            low_points
        })
}

fn calc_risk_level_sum(map: &HeightMap, low_points: &Vec<Coords>) -> u32 {
    low_points
        .iter()
        .fold(0, |current_sum, (y, x)| current_sum + map[*y][*x] + 1)
}

pub fn solve_p1(data: &str) -> u32 {
    let heightmap = parse_input(data);
    calc_risk_level_sum(&heightmap, &find_low_points(&heightmap))
}

pub fn solve_p2(_data: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let data = r"2199943210
3987894921
9856789892
8767896789
9899965678
";
        assert_eq!(solve_p1(data), 15);
    }

    #[test]
    fn test_p2() {
        let data = r"2199943210
3987894921
9856789892
8767896789
9899965678
";
        assert_eq!(solve_p2(data), 1134);
    }
}
