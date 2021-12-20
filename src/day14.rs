use std::collections::{BTreeMap, HashMap};

pub fn first_part(xs: &mut Vec<u8>, rules: &HashMap<Vec<u8>, u8>) -> i32 {
    for _ in 0..10 {
        let to_insert: BTreeMap<usize, u8> = xs
            .windows(2)
            .zip(0..xs.len())
            .flat_map(|(slice, idx)| {
                if rules.contains_key(slice) {
                    Some((idx, *rules.get(slice).unwrap()))
                } else {
                    None
                }
            })
            .collect();
        for (&k, &v) in to_insert.iter().rev() {
            xs.insert(k + 1, v);
        }
    }

    let freq = xs.iter().fold(BTreeMap::new(), |mut btm, c| {
        *btm.entry(c).or_insert(0) += 1;
        btm
    });
    let mut most = i32::MIN;
    let mut least = i32::MAX;
    for (_, v) in freq {
        most = most.max(v);
        least = least.min(v);
    }
    most - least
}

// 统计以当前字符开始的规则的数目
// 根据规则数目，计算字符数
// 动态规划
pub fn second_part(xs: &mut Vec<u8>, rules: &HashMap<Vec<u8>, u8>) -> i64 {
    const N: usize = 26;
    let mut rule_count = vec![vec![0i64; N]; N];
    for i in 1..xs.len() {
        rule_count[idx(xs[i - 1])][idx(xs[i])] += 1;
    }
    for _ in 0..40 {
        let mut rule_count_next = vec![vec![0i64; N]; N];
        for i in b'A'..=b'Z' {
            for j in b'A'..=b'Z' {
                if rules.contains_key(&vec![i, j]) {
                    let k = *rules.get(&vec![i, j]).unwrap();
                    rule_count_next[idx(i)][idx(k)] += rule_count[idx(i)][idx(j)];
                    rule_count_next[idx(k)][idx(j)] += rule_count[idx(i)][idx(j)];
                } else {
                    rule_count_next[idx(i)][idx(j)] = rule_count[idx(i)][idx(j)];
                }
            }
        }
        rule_count = rule_count_next;
    }
    let mut freq = vec![0i64; N];
    for i in 0..N {
        for j in 0..N {
            freq[i] += rule_count[i][j];
        }
        // 注意最后一个字符不会动，现在计数进去
        if i == idx(*xs.last().unwrap()) as usize {
            freq[i] += 1;
        }
    }
    freq.iter().max().unwrap() - freq.iter().filter(|&d| *d > 0).min().unwrap()
}

fn idx(b: u8) -> usize {
    (b - b'A') as usize
}

pub fn read_rules(s: &str) -> (Vec<u8>, HashMap<Vec<u8>, u8>) {
    let mut xs: Vec<u8> = vec![];
    let mut rules: HashMap<Vec<u8>, u8> = HashMap::new();
    let mut flag = true;
    for line in s.lines() {
        if line.is_empty() {
            continue;
        }
        if flag {
            xs = line.as_bytes().to_vec();
            flag = false;
        } else {
            let ss: Vec<&str> = line.split(" -> ").collect();
            rules.insert(ss[0].as_bytes().to_vec(), ss[1].as_bytes().to_vec()[0]);
        }
    }
    (xs, rules)
}

#[cfg(test)]
mod tests {
    use super::first_part;
    use super::read_rules;
    use crate::day14::second_part;

    #[test]
    fn first_part_works() {
        let s = "NNCB

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
CN -> C";
        let (mut xs, rules) = read_rules(s);
        assert_eq!(first_part(&mut xs, &rules), 1588);
    }

    #[test]
    fn second_part_works() {
        let s = "NNCB

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
CN -> C";
        let (mut xs, rules) = read_rules(s);
        assert_eq!(second_part(&mut xs, &rules), 2188189693529);
    }
}
