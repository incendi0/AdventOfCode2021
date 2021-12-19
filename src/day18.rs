#[derive(Debug)]
pub enum Element {
    Int(i32),
    Pair([Box<Element>; 2]),
}

fn parse(s: &str) -> Element {
    let mut curr_idx = 0;
    do_parse(s.as_bytes(), &mut curr_idx)
}

fn do_parse(s: &[u8], curr_idx: &mut usize) -> Element {
    if s[*curr_idx] == b'[' {
        *curr_idx += 1;
        let first = do_parse(s, curr_idx);
        *curr_idx += 1; // ,
        let second = do_parse(s, curr_idx);
        *curr_idx += 1;
        Element::Pair([Box::new(first), Box::new(second)])
    } else {
        do_parse_int(s, curr_idx)
    }
}

fn do_parse_int(s: &[u8], curr_idx: &mut usize) -> Element {
    let mut ret: i32 = 0;
    while *curr_idx < s.len() && s[*curr_idx].is_ascii_digit() {
        ret = ret * 10 + (s[*curr_idx] - b'0') as i32;
        *curr_idx += 1;
    }
    Element::Int(ret)
}

#[cfg(test)]
mod tests {
    use crate::day18::Element;
    use super::parse;

    #[test]
    fn parse_works() {
        let s = "[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        let xs: Vec<&str> = s.lines().collect();
        // for s in xs {
        //     println!("{:?}", s);
        //     println!("{:?}", parse(s));
        // }
    }
}