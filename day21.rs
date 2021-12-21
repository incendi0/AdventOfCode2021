use std::collections::HashMap;

pub fn first_part(first: i32, second: i32) -> i32 {
    let (mut first_pos, mut second_pos) = (first, second);
    let (mut first_score, mut second_score) = (0, 0);
    let mut die = 1;
    loop {
        first_pos = (first_pos + (die % 100 + (die + 1) % 100 + (die + 2) % 100) % 10) % 10;
        die += 3;
        first_score += if first_pos == 0 { 10 } else { first_pos };
        if first_score >= 1000 {
            break;
        }
        second_pos = (second_pos + (die % 100 + (die + 1) % 100 + (die + 2) % 100) % 10) % 10;
        die += 3;
        second_score += if second_pos == 0 { 10 } else { second_pos };
        if second_score >= 1000 {
            break;
        }
    }
    first_score.min(second_score) * (die - 1)
}

pub fn second_part(first: usize, second: usize) -> i64 {
    let mut cache: HashMap<(usize, i64, usize, i64), [i64; 2]> = HashMap::new();
    solve(first - 1, 0, second - 1, 0, &mut cache).into_iter().max().unwrap()
}

fn solve(first: usize, first_score: i64, second: usize, second_score: i64, cache: &mut HashMap<(usize, i64, usize, i64), [i64; 2]>) -> [i64; 2] {
    if first_score >= 21 {
        return [1, 0];
    }
    if second_score >= 21 {
        return [0, 1];
    }
    if cache.contains_key(&(first, first_score, second, second_score)) {
        return cache[&(first, first_score, second, second_score)];
    }
    let mut ret = [0, 0];
    for d1 in [1, 2, 3] {
        for d2 in [1, 2, 3] {
            for d3 in [1, 2, 3] {
                let new_pos = (first + d1 + d2 + d3) % 10;
                let new_score = first_score + new_pos as i64 + 1;
                let [s, f] = solve(second, second_score, new_pos, new_score, cache);
                ret[0] += f;
                ret[1] += s;
            }
        }
    }
    cache.insert((first, first_score, second, second_score), ret);
    ret
}

#[cfg(test)]
mod tests {
    use super::first_part;
    use super::second_part;

    #[test]
    fn first_part_works() {
        println!("{:?}", first_part(4, 8));
    }

    #[test]
    fn second_part_works() {
        println!("{:?}", second_part(4, 8))
    }
}
