pub fn solve_p1(data: &str) -> usize {
    parse_input(data).iter().fold(0, |count, (_, output)| {
        count
            + output
                .iter()
                .filter(|x| match x.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
    })
}

pub fn solve_p2(data: &str) -> i32 {
    println!("NO IMPLENTED YET!");
    parse_input(data).iter().fold(0, |output_sum, line_tuple| {
        output_sum + calc_output_value(&line_tuple.0, &line_tuple.1)
    })
}

/*
 * used by p1 and p2
 */

fn parse_input(data: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    data.lines()
        .map(|line| {
            let split_line = line.split_once(" | ").unwrap();
            (
                split_line.0.split_ascii_whitespace().collect(),
                split_line.1.split_ascii_whitespace().collect(),
            )
        })
        .collect()
}

/*
 * used by p2
 */

fn calc_output_value(signal: &Vec<&str>, _output: &Vec<&str>) -> i32 {
    let (one, four, seven, eight) = determine_unique_codes(&signal);
    let segment_a = find_diff(&one, &seven)[0].clone();
    println!(
        "1={:?}, 4={:?}, 7={:?}, 8={:?}, a={:?}",
        one, four, seven, eight, segment_a
    );
    1
}

fn determine_unique_codes(signal: &Vec<&str>) -> (String, String, String, String) {
    (
        signal.iter().find(|x| x.len() == 2).unwrap().to_string(), // one
        signal.iter().find(|x| x.len() == 4).unwrap().to_string(), // four
        signal.iter().find(|x| x.len() == 3).unwrap().to_string(), // seven
        signal.iter().find(|x| x.len() == 7).unwrap().to_string(), // eight
    )
}

fn find_diff(a: &str, b: &str) -> Vec<char> {
    a.chars()
        .into_iter()
        .chain(b.chars().into_iter())
        .into_iter()
        .fold(Vec::new(), |mut diffs, char| {
            if a.chars().into_iter().filter(|c| c == &char).count() == 1 {
                diffs
            } else {
                diffs.push(char);
                diffs
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let data = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
        assert_eq!(solve_p1(data), 26);
    }

    // #[test]
    // fn test_p2() {
    //     let data = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    // edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    // fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    // fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    // aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    // fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    // dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    // bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    // egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    // gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    // ";
    //     assert_eq!(solve_p2(data), 61229);
    // }
}
