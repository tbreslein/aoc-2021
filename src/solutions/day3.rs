/*
 * Used by both solve_p1 and solve_p2
 */

fn data_to_bit_matrix(data: &str) -> Vec<Vec<u32>> {
    data.lines().fold(Vec::new(), |mut row, line| {
        row.push(line.chars().fold(Vec::new(), |mut col, c| {
            col.push(c.to_digit(2).unwrap());
            col
        }));
        row
    })
}

fn bits_to_int(bits: Vec<u32>) -> i32 {
    let length = bits.len();
    bits.iter().enumerate().fold(0, |mut acc, (i, bit)| {
        if bit > &0 {
            acc += 2_i32.pow((length - 1 - i).try_into().unwrap())
        }
        acc
    })
}

pub fn solve_p1(data: &str) -> i32 {
    let bit_matrix = data_to_bit_matrix(data);
    let num_cols = bit_matrix[0].len();

    let gamma_bits: Vec<u32> = bit_matrix
        .iter()
        .fold(vec![0; num_cols], |mut acc, line| {
            line.iter().enumerate().for_each(|(i, bit)| match bit {
                0 => acc[i] -= 1,
                1 => acc[i] += 1,
                _ => panic!("this SHOULD be impossible..."),
            });
            acc
        })
        .iter()
        .map(|b| if b > &0 { 1 } else { 0 })
        .collect();

    let epsilon_bits: Vec<u32> = gamma_bits
        .iter()
        .map(|b| match b {
            0 => 1,
            1 => 0,
            _ => panic!("Parsing error!"),
        })
        .collect();

    bits_to_int(gamma_bits) * bits_to_int(epsilon_bits)
}

/*
 * Used by solve_p2 only
 */

fn get_count_at_i(matrix: &Vec<Vec<u32>>, i: usize) -> i32 {
    matrix.iter().fold(0_i32, |mut acc, line| {
        match line[i] {
            0 => acc -= 1,
            1 => acc += 1,
            _ => panic!("this SHOULD be impossible..."),
        };
        acc
    })
}

fn filter_with_predicate_at_i(
    matrix: &Vec<Vec<u32>>,
    f: fn(i32) -> bool,
    i: usize,
) -> Vec<Vec<u32>> {
    let bit = match f(get_count_at_i(&matrix, i)) {
        true => 1,
        false => 0,
    };
    matrix.iter().fold(Vec::new(), |mut acc, line| {
        if line[i] == bit {
            acc.push(line.to_vec());
        }
        acc
    })
}

pub fn solve_p2(data: &str) -> i32 {
    let mut oxygen_matrix = data_to_bit_matrix(data);
    let mut co2_matrix = oxygen_matrix.clone();
    let num_cols = oxygen_matrix[0].len();

    // I could abstract these two for loops away too, but I think that would actually
    // hurt the readability of this function

    for i in 0..num_cols {
        oxygen_matrix = filter_with_predicate_at_i(&oxygen_matrix, |x| x >= 0, i);
        if oxygen_matrix.len() == 1 {
            break;
        }
    }

    for i in 0..num_cols {
        co2_matrix = filter_with_predicate_at_i(&co2_matrix, |x| x < 0, i);
        if co2_matrix.len() == 1 {
            break;
        }
    }

    bits_to_int(oxygen_matrix.pop().unwrap()) * bits_to_int(co2_matrix.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let data =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
        assert_eq!(solve_p1(data), 198);
    }
    #[test]
    fn test_p2() {
        let data =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
        assert_eq!(solve_p2(data), 230);
    }
}
