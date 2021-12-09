use std::collections::{HashMap, HashSet};

pub fn first_part(xs: &Vec<String>) -> i32 {
    let mut total = 0;
    for s in xs {
        let ss = s.split('|').collect::<Vec<&str>>();
        for output in ss[1].split_ascii_whitespace() {
            if output.len() == 2 || output.len() == 3 || output.len() == 4 || output.len() == 7 {
                total += 1;
            }
        }
    }
    total
}

pub fn second_part(xs: &Vec<String>) -> i32 {
    xs.iter().map(|s| calculate_entry(s)).sum()
}

fn calculate_entry(s: &str) -> i32 {
    let xs: Vec<&str> = s.split('|').collect();
    let mut shuffled: Vec<&str> = xs[0].split_ascii_whitespace().collect();
    let one = remove_len(&mut shuffled, 2);
    let four = remove_len(&mut shuffled, 4);
    let seven = remove_len(&mut shuffled, 3);
    let eight = remove_len(&mut shuffled, 7);
    // 0, 6, 9根据1，获取6
    let mut six_idx = 0;
    for (i, s) in shuffled.iter().enumerate() {
        if s.len() == 6 && !contains(s, &one) {
            six_idx = i;
            break;
        }
    }
    let six = remove_idx(&mut shuffled, six_idx);
    let mut three_idx = 0;
    for (i, s) in shuffled.iter().enumerate() {
        if s.len() == 5 && contains(s, &one) {
            three_idx = i;
            break;
        }
    }
    let three = remove_idx(&mut shuffled, three_idx);
    let mut nine_idx = 0;
    let mut five_idx = 0;
    'outer:
    for i in 0..shuffled.len() {
        for j in 0..shuffled.len() {
            if i != j && contains(shuffled[i], &shuffled[j].as_bytes().to_vec()) {
                nine_idx = i;
                five_idx = j;
                break 'outer;
            }
        }
    }
    // nine_idx is bigger
    let nine: Vec<u8>;
    let five: Vec<u8>;
    if nine_idx > five_idx {
        nine = remove_idx(&mut shuffled, nine_idx);
        five = remove_idx(&mut shuffled, five_idx);
    } else {
        five = remove_idx(&mut shuffled, five_idx);
        nine = remove_idx(&mut shuffled, nine_idx);
    }
    let two_idx: usize;
    if shuffled[0].len() == 5 {
        two_idx = 0;
    } else {
        two_idx = 1;
    }
    let two = remove_idx(&mut shuffled, two_idx);
    let zero = remove_idx(&mut shuffled, 0);

    let arr = vec![zero, one, two, three, four, five, six, seven, eight, nine];
    let mut mapping = HashMap::new();
    for (i, num) in arr.into_iter().enumerate() {
        mapping.insert(num, i as i32);
    }
    let mut total = 0;
    for s in xs[1].split_ascii_whitespace() {
        let mut vs = s.as_bytes().to_vec();
        vs.sort();
        total = total * 10 + mapping.get(&vs).unwrap();
    }
    total
}

fn contains(haystack: &str, needle: &Vec<u8>) -> bool {
    let haystack_set: HashSet<&u8> = haystack.as_bytes().iter().collect::<HashSet<&u8>>();
    let needle_set: HashSet<&u8> = needle.iter().collect();
    let ret = haystack_set.is_superset(&needle_set);

    ret
}

fn remove_len(xs: &mut Vec<&str>, len: usize) -> Vec<u8> {
    let mut idx = 0;
    for (i, s) in xs.iter().enumerate() {
        if s.len() == len {
            idx = i;
            break;
        }
    }
    let mut ret = xs.remove(idx).as_bytes().to_vec();
    ret.sort();
    ret
}

fn remove_idx(xs: &mut Vec<&str>, idx: usize) -> Vec<u8> {
    let mut ret = xs.remove(idx).as_bytes().to_vec();
    ret.sort();
    ret
}

#[cfg(test)]
mod tests {
    use super::first_part;
    use super::second_part;

    #[test]
    fn first_part_works() {
        let s = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce";
        let xs = s.split('\n').map(|s| s.to_string()).collect();
        assert_eq!(first_part(&xs), 26);
    }

    #[test]
    fn second_part_works() {
        let s = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let xs = s.split('\n').map(|s| s.to_string()).collect();
        assert_eq!(second_part(&xs), 5353);
    }

    #[test]
    fn second_part_works2() {
        let s = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce";
        let xs = s.split('\n').map(|s| s.to_string()).collect();
        assert_eq!(second_part(&xs), 61229);
    }
}
