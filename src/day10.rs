use std::collections::{HashMap, HashSet, VecDeque};

// 不要想太复杂，corrupted的排列不用管
// 使用栈配对就行
pub fn first_part(xs: &Vec<String>) -> i32 {
    xs.iter().map(|s| illegal_point(s)).sum()
}

pub fn second_part(xs: &Vec<String>) -> i64 {
    let mut points: Vec<i64> = xs
        .iter()
        .map(|s| illegal_point2(s))
        .filter(|&p| p > 0)
        .collect();
    points.sort_unstable();
    points[points.len() / 2]
}

fn illegal_point2(s: &str) -> i64 {
    let mut stack = VecDeque::new();
    let open_set = HashSet::from(['(', '[', '{', '<']);
    let close_open_mapping = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let open_close_mapping = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let points_mapping = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    for ch in s.chars() {
        if open_set.contains(&ch) {
            stack.push_back(ch);
        } else if stack.is_empty() || stack.back().unwrap() != close_open_mapping.get(&ch).unwrap()
        {
            return 0;
        } else {
            stack.pop_back();
        }
    }
    if stack.is_empty() {
        return 0;
    }
    let mut total = 0;
    while !stack.is_empty() {
        let p = points_mapping
            .get(open_close_mapping.get(&stack.pop_back().unwrap()).unwrap())
            .unwrap();
        total = total * 5 + p;
    }
    total
}

fn illegal_point(s: &str) -> i32 {
    let mut stack = VecDeque::new();
    let open_set = HashSet::from(['(', '[', '{', '<']);
    // let close_set = HashSet::from([')', ']', '}', '>']);
    let close_open_mapping = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let points_mapping = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut total = 0;
    for ch in s.chars() {
        if open_set.contains(&ch) {
            stack.push_back(ch);
        } else if stack.is_empty() || stack.back().unwrap() != close_open_mapping.get(&ch).unwrap()
        {
            total += points_mapping.get(&ch).unwrap_or(&0);
            break;
        } else {
            stack.pop_back();
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::first_part;
    use super::second_part;

    #[test]
    fn first_part_works() {
        let s = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let xs = s.split('\n').map(|s| s.to_string()).collect();
        assert_eq!(first_part(&xs), 26397);
    }

    #[test]
    fn second_part_works() {
        let s = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let xs = s.split('\n').map(|s| s.to_string()).collect();
        assert_eq!(second_part(&xs), 288957);
    }
}
