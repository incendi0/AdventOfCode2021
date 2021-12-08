// 选取中位数
pub fn first_part(xs: &mut Vec<i32>) -> i32 {
    xs.sort();
    let n = xs.len();
    if n % 2 == 1 {
        calculate_cost(xs, n / 2)
    } else {
        calculate_cost(xs, (n - 1) / 2).min(calculate_cost(xs, (n - 1) / 2 + 1))
    }
}

// 选取平均值左右
pub fn second_part(xs: &mut Vec<i32>) -> i32 {
    xs.sort();
    let n = xs.len();
    let mean: i32 = xs.iter().sum::<i32>() / n as i32;
    let mut ret = i32::MAX;
    for m in [mean - 1, mean, mean + 1] {
        let mut total = 0;
        for x in xs.iter() {
            total += gauss_sum((m - *x).abs());
        }
        ret = ret.min(total);
    }
    ret
}

fn gauss_sum(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn calculate_cost(xs: &Vec<i32>, idx: usize) -> i32 {
    let mut total = 0;
    for x in xs {
        total += (x - xs[idx]).abs();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::first_part;
    use super::second_part;

    #[test]
    fn first_part_works() {
        let mut xs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        // let mut xs = vec![3];
        assert_eq!(first_part(&mut xs), 37);
    }

    #[test]
    fn second_part_works() {
        let mut xs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        // let mut xs = vec![3];
        assert_eq!(second_part(&mut xs), 168);
    }
}
