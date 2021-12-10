enum LineState {
    Corrupt(String),
    Incomplete(Vec<String>),
    Complete,
}

fn parse_input(data: &str) -> Vec<Vec<String>> {
    data.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(line.chars().map(|c| c.to_string()).collect());
        acc
    })
}

fn is_corrupted(open_char: &str, closing_char: &str) -> bool {
    (open_char == "(" && closing_char != ")")
        || (open_char == "[" && closing_char != "]")
        || (open_char == "{" && closing_char != "}")
        || (open_char == "<" && closing_char != ">")
}

fn convert_corrupted_char(illegal_char: &str) -> u32 {
    match illegal_char {
        ")" => 3,
        "]" => 57,
        "}" => 1197,
        ">" => 25137,
        _ => panic!("jesus, how..."),
    }
}

fn is_open_char(c: &str) -> bool {
    match c {
        "(" | "[" | "{" | "<" => true,
        _ => false,
    }
}

fn try_line(line: &Vec<String>) -> LineState {
    let mut stack = Vec::new();
    for c in line.into_iter() {
        if is_open_char(c) {
            stack.push(c.clone());
        } else if is_corrupted(&stack.pop().unwrap(), c) {
            return LineState::Corrupt(c.to_string());
        }
    }
    if stack.len() > 0 {
        return LineState::Incomplete(stack);
    }
    return LineState::Complete;
}

pub fn solve_p1(data: &str) -> u32 {
    parse_input(data)
        .iter()
        .fold(Vec::new(), |mut corrupt_chars, line| {
            if let LineState::Corrupt(c) = try_line(line) {
                corrupt_chars.push(convert_corrupted_char(&c));
            }
            corrupt_chars
        })
        .into_iter()
        .sum()
}

pub fn solve_p2(data: &str) -> u64 {
    let mut scores = parse_input(data)
        .iter()
        .fold(Vec::new(), |mut autocomplete_scores, line| {
            if let LineState::Incomplete(remaining_chars) = try_line(&line) {
                autocomplete_scores.push(remaining_chars.into_iter().rev().fold(0, |score, c| {
                    if c == "(" {
                        score * 5 + 1
                    } else if c == "[" {
                        score * 5 + 2
                    } else if c == "{" {
                        score * 5 + 3
                    } else if c == "<" {
                        score * 5 + 4
                    } else {
                        score
                    }
                }));
            }
            autocomplete_scores
        });
    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let data = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
        assert_eq!(solve_p1(data), 26397);
    }

    #[test]
    fn test_p2() {
        let data = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
        assert_eq!(solve_p2(data), 288957);
    }
}
