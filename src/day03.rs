pub fn first_part(xs: &[String]) -> i32 {
    let (n, m) = (xs.len(), xs[0].len());
    let count = count_bit_1(xs);
    let s: String = count
        .iter()
        .map(|&x| if x > n / 2 { '1' } else { '0' })
        .collect();
    let u = usize::from_str_radix(&s, 2).unwrap();
    ((usize::pow(2, m as u32) - 1 - u) * u) as i32
}

pub fn second_part(xs: &[String]) -> i32 {
    let oxygen = find_rate(xs, true);
    let co2 = find_rate(xs, false);
    oxygen * co2
}

fn find_rate(xs: &[String], oxygen: bool) -> i32 {
    let m = xs[0].len();
    let mut ss: Vec<&[u8]> = xs.iter().map(|s| s.as_bytes()).collect();
    'outer: loop {
        for i in 0..m {
            let n = ss.len();
            let mut count = 0;
            for s in ss.iter() {
                if s[i] == b'1' {
                    count += 1;
                }
            }
            ss.retain(|&s| {
                if n <= 2 * count {
                    s[i] == if oxygen { b'1' } else { b'0' }
                } else {
                    s[i] == if oxygen { b'0' } else { b'1' }
                }
            });
            if ss.len() == 1 {
                break 'outer;
            }
        }
    }
    let s = String::from_utf8(Vec::from(ss[0])).expect("Invalid UTF");
    i32::from_str_radix(&s, 2).unwrap()
}

fn count_bit_1(xs: &[String]) -> Vec<usize> {
    let m = xs[0].len();
    let mut count: Vec<usize> = vec![];
    count.resize(m, 0);
    for s in xs {
        for (i, &ch) in s.as_bytes().iter().enumerate() {
            if ch == b'1' {
                count[i] += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::first_part;
    use super::second_part;

    #[test]
    fn first_part_works() {
        let s = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let xs: Vec<String> = s.split_ascii_whitespace().map(|s| s.to_string()).collect();
        assert_eq!(first_part(&xs), 198)
    }

    #[test]
    fn second_part_works() {
        let s = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let xs: Vec<String> = s.split_ascii_whitespace().map(|s| s.to_string()).collect();
        assert_eq!(second_part(&xs), 230)
    }
}
