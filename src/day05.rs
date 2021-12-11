use std::collections::HashMap;

pub fn count_overlaps(xs: &Vec<String>, is_first_part: bool) -> i32 {
    let mut freq = HashMap::new();

    for s in xs.iter() {
        let points: Vec<&str> = s.split(" -> ").collect();
        let start: Vec<i32> = points[0]
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let end: Vec<i32> = points[1]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if start[0] == end[0] {
            let (l, r) = (start[1].min(end[1]), start[1].max(end[1]));
            for y in l..=r {
                let count = freq.entry((start[0], y)).or_insert(0);
                *count += 1;
            }
        } else if start[1] == end[1] {
            let (l, r) = (start[0].min(end[0]), start[0].max(end[0]));
            for x in l..=r {
                let count = freq.entry((x, start[1])).or_insert(0);
                *count += 1;
            }
        } else if !is_first_part {
            let (l, r) = (start[1].min(end[1]), start[1].max(end[1]));
            let denom = end[1] - start[1];
            for y in l..=r {
                let num = end[0] * y - end[0] * start[1] - start[0] * y + start[0] * end[1];
                if num % denom == 0 {
                    let x = num / denom;
                    let count = freq.entry((x, y)).or_insert(0);
                    *count += 1;
                }
            }
        }
    }
    freq.values().filter(|v| **v > 1).count() as i32
}

#[cfg(test)]
mod tests {
    use super::count_overlaps;

    #[test]
    fn first_part_works() {
        let xs = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        assert_eq!(count_overlaps(&xs, true), 5);
    }

    #[test]
    fn second_part_works() {
        let xs = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        assert_eq!(count_overlaps(&xs, false), 12);
    }
}
