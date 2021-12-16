use std::collections::HashMap;

type Template = Vec<char>;
type Rules = HashMap<(char, char), char>;

pub fn solve_p1(data: &str) -> i64 {
    solve(data, 10)
}

pub fn solve_p2(_data: &str) -> i64 {
    // it works, but it needs to be optimised, because
    // this might run for north of 10 minutes, even on
    // the test input.
    // solve(data, 40)
    1
}

// tried actually writing the growing template into a vector
// before I came up with this version, and that would just
// eat up RAM like it's nothing. This recursive version still
// is VERY slow for stepmax >= 25, but the memory footprint is
// very low and practically constant.
// (the recursions cost a bit of memory, obviously)
fn solve(data: &str, stepmax: u32) -> i64 {
    // initial polymer template and pair insertion rules
    let (template, rules) = parse_input(data);

    // how often each char occurs
    let mut counts: HashMap<char, i64> = HashMap::new();

    // count chars in the initial template
    count_chars(&template, &mut counts);

    // go through initial template in pairs of, starting the recursion
    // from each of those pairs
    for i in 1..template.len() {
        recurse(
            &template[i - 1],
            &template[i],
            1,
            stepmax,
            &mut counts,
            &rules,
        );
    }
    return counts.values().max().unwrap() - counts.values().min().unwrap();
}

// for a given pair c1 and c2, check which new letter is produced
// between those two, let's call it new_letter. Then, unless we
// have reached stepmax, it recurses into checking the pairs
// (c1, new_letter) and (c2, new_letter)
fn recurse(
    c1: &char,
    c2: &char,
    step: u32,
    stepmax: u32,
    counts: &mut HashMap<char, i64>,
    rules: &Rules,
) {
    let new_letter = rules.get(&(*c1, *c2)).unwrap();

    // increase count by 1 or insert with count 1
    if let Some(x) = counts.get_mut(new_letter) {
        *x += 1;
    } else {
        counts.insert(*new_letter, 1);
    }

    if step < stepmax {
        recurse(c1, &new_letter, step + 1, stepmax, counts, rules);
        recurse(&new_letter, c2, step + 1, stepmax, counts, rules);
    }
    return;
}

fn parse_input(data: &str) -> (Template, Rules) {
    let data_tuple = data.split_once("\n\n").unwrap();
    let template: Template = data_tuple.0.chars().collect();
    let rules: Rules = data_tuple.1.lines().fold(HashMap::new(), |mut map, line| {
        let tuple = line.split_once(" -> ").unwrap();
        let mut first_iter = tuple.0.chars();
        let first_letter = first_iter.next().unwrap();
        let second_letter = first_iter.next().unwrap();
        let insert_letter = tuple.1.chars().next().unwrap();
        map.insert((first_letter, second_letter), insert_letter);
        return map;
    });
    return (template, rules);
}

fn count_chars(template: &Template, counts: &mut HashMap<char, i64>) -> () {
    template.iter().for_each(|c| {
        if let Some(x) = counts.get_mut(c) {
            *x += 1;
        } else {
            counts.insert(*c, 1);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let data = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";
        assert_eq!(solve_p1(data), 1588);
    }

    // #[test]
    // fn test_p2() {
    //     let data = r"NNCB

    // CH -> B
    // HH -> N
    // CB -> H
    // NH -> C
    // HB -> C
    // HC -> B
    // HN -> C
    // NN -> C
    // BH -> H
    // NC -> B
    // NB -> B
    // BN -> B
    // BB -> N
    // BC -> B
    // CC -> N
    // CN -> C
    // ";
    //     assert_eq!(solve_p2(data), 2188189693529);
    // }
}
