use std::collections::HashMap;

// 模拟剩余生产天数的成熟鱼和新生鱼
// 复杂点的兔子问题，类fibonacci数
pub fn simulate(xs: &Vec<i32>, duration: i32) -> i64 {
    let mut count = HashMap::new();
    let mut total = 0;
    for x in xs {
        let freq = count
            .entry(x)
            .or_insert_with(|| total_with_duration(duration - x - 1));
        total += *freq;
    }
    total
}

fn total_with_duration(duration: i32) -> i64 {
    const N: usize = 7;
    const M: usize = 9;
    let mut mature = [0_i64; N];
    let mut young = [0_i64; M];
    mature[N - 1] = 1;
    young[M - 1] = 1;
    for _ in 0..duration {
        let added_young = mature[0];
        for j in 0..N - 1 {
            mature[j] = mature[j + 1];
        }
        let new_mature = young[0];
        mature[N - 1] = added_young + new_mature;
        for j in 0..M - 1 {
            young[j] = young[j + 1];
        }
        young[M - 1] = added_young + new_mature;
    }
    young.iter().sum::<i64>() + mature.iter().sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::simulate;

    #[test]
    fn first_part_works() {
        let mut xs = vec![3, 4, 3, 1, 2];
        assert_eq!(simulate(&mut xs, 18), 26);
        assert_eq!(simulate(&mut xs, 80), 5934);
    }
}
