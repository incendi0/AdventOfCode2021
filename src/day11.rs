use std::collections::VecDeque;

pub fn first_part(xs: &mut Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for _ in 0..100 {
        total += update(xs);
    }
    total
}

pub fn second_part(xs: &mut Vec<Vec<i32>>) -> i32 {
    let (m, n) = (xs.len(), xs[0].len());
    let mut ret = 0;
    loop {
        let flashes = update(xs);
        ret += 1;
        if flashes == (m * n) as i32 {
            return ret;
        }
    }
}

fn update(xs: &mut Vec<Vec<i32>>) -> i32 {
    let (m, n) = (xs.len(), xs[0].len());
    let mut queue = VecDeque::new();
    for i in 0..m {
        for j in 0..n {
            xs[i][j] += 1;
            if xs[i][j] == 10 {
                queue.push_back((i, j));
            }
        }
    }
    while !queue.is_empty() {
        let (x, y) = queue.pop_back().unwrap();
        for (dx, dy) in [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (-1, -1),
            (-1, 1),
            (1, 1),
            (1, -1),
        ] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if xs[nx][ny] < 10 {
                    if xs[nx][ny] == 9 {
                        queue.push_back((nx, ny));
                    }
                    xs[nx][ny] += 1;
                }
            }
        }
    }

    let mut total = 0;
    for i in 0..m {
        for j in 0..n {
            if xs[i][j] >= 10 {
                xs[i][j] = 0;
                total += 1;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::first_part;
    use super::second_part;
    use crate::day09::read_to_matrix;

    #[test]
    fn first_part_works() {
        let s = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut xs = read_to_matrix(s);
        assert_eq!(first_part(&mut xs), 1656);
    }

    #[test]
    fn second_part_works() {
        let s = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut xs = read_to_matrix(s);
        assert_eq!(second_part(&mut xs), 195);
    }
}
