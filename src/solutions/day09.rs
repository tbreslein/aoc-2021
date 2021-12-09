use std::collections::HashSet;

type HeightMap = Vec<Vec<u32>>;
type Coords = (usize, usize);

/*
 * used by p1 and p2
 */

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

/*
 * p1 only
 */

fn calc_risk_level_sum(map: &HeightMap, low_points: &Vec<Coords>) -> u32 {
    low_points
        .iter()
        .fold(0, |current_sum, (y, x)| current_sum + map[*y][*x] + 1)
}

pub fn solve_p1(data: &str) -> u32 {
    let heightmap = parse_input(data);
    calc_risk_level_sum(&heightmap, &find_low_points(&heightmap))
}

/*
 * p2 only
 */

fn fill_basin_set(map: &HeightMap, point: &Coords, basin_set: &mut HashSet<Coords>) -> () {
    basin_set.insert(point.clone());
    // check up and recurse
    if point.0 > 0 && !basin_set.contains(&(point.0 - 1, point.1)) && map[point.0 - 1][point.1] < 9
    {
        fill_basin_set(&map, &(point.0 - 1, point.1), basin_set);
    }

    // check down and recurse
    if point.0 < map.len() - 1
        && !basin_set.contains(&(point.0 + 1, point.1))
        && map[point.0 + 1][point.1] < 9
    {
        fill_basin_set(&map, &(point.0 + 1, point.1), basin_set);
    }

    // check left and recurse
    if point.1 > 0 && !basin_set.contains(&(point.0, point.1 - 1)) && map[point.0][point.1 - 1] < 9
    {
        fill_basin_set(&map, &(point.0, point.1 - 1), basin_set);
    }

    // check right and recurse
    if point.1 < map[0].len() - 1
        && !basin_set.contains(&(point.0, point.1 + 1))
        && map[point.0][point.1 + 1] < 9
    {
        fill_basin_set(&map, &(point.0, point.1 + 1), basin_set);
    }
}

fn calc_basin_sizes(map: &HeightMap, low_points: &Vec<Coords>) -> Vec<usize> {
    let mut basin_sizes = low_points
        .iter()
        .fold(Vec::new(), |mut basin_sizes, point| {
            let mut basin_set: HashSet<Coords> = HashSet::new();
            fill_basin_set(&map, &point, &mut basin_set);
            basin_sizes.push(basin_set.len());
            basin_sizes
        });
    basin_sizes.sort();
    basin_sizes
}

pub fn solve_p2(data: &str) -> usize {
    let heightmap = parse_input(data);
    calc_basin_sizes(&heightmap, &find_low_points(&heightmap))
        .iter()
        .rev()
        .take(3)
        .product()
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
