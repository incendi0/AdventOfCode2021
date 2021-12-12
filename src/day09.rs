use std::collections::VecDeque;

// 第二部分不用想太多，找到最低点，bfs就行
// 其实并查集也行，记录下思路
pub fn first_part(matrix: &Vec<Vec<i32>>) -> i32 {
    let lowest = find_lowest(matrix);
    lowest.iter().map(|&(x, y)| matrix[x][y] + 1).sum()
}

pub fn second_part(matrix: &Vec<Vec<i32>>) -> i32 {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut visited = vec![vec![false; n]; m];
    let lowest = find_lowest(matrix);
    let mut total = vec![];
    for coord in lowest.iter() {
        total.push(basin_count(matrix, *coord, &mut visited));
    }
    total.sort_unstable();
    total.iter().rev().take(3).product()
}

fn basin_count(matrix: &Vec<Vec<i32>>, start: (usize, usize), visited: &mut Vec<Vec<bool>>) -> i32 {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut count = 1;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    visited[start.0][start.1] = true;
    q.push_back(start);
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let (nx, ny) = (u.0 as i32 + dx, u.1 as i32 + dy);
            if nx >= 0
                && nx < m as i32
                && ny >= 0
                && ny < n as i32
                && !visited[nx as usize][ny as usize]
                && matrix[nx as usize][ny as usize] != 9
            {
                count += 1;
                visited[nx as usize][ny as usize] = true;
                q.push_back((nx as usize, ny as usize));
            }
        }
    }
    count
}

fn find_lowest(matrix: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut ret = vec![];
    for i in 0..m {
        for j in 0..n {
            let mut is_lowest = true;
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                let (nx, ny) = (i as i32 + dx, j as i32 + dy);
                if nx >= 0
                    && nx < m as i32
                    && ny >= 0
                    && ny < n as i32
                    && matrix[nx as usize][ny as usize] <= matrix[i][j]
                {
                    is_lowest = false;
                    break;
                }
            }
            if is_lowest {
                ret.push((i, j));
            }
        }
    }
    ret
}

pub fn read_to_matrix(s: &str) -> Vec<Vec<i32>> {
    let xs: Vec<&str> = s.split_ascii_whitespace().collect();
    let (m, n) = (xs.len(), xs[0].len());
    let mut matrix = vec![vec![0; n]; m];
    for i in 0..m {
        for (j, &ch) in xs[i].as_bytes().iter().enumerate() {
            matrix[i][j] = ch as i32 - b'0' as i32;
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::first_part;
    use super::read_to_matrix;
    use super::second_part;

    #[test]
    fn first_part_works() {
        let s = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let xs = read_to_matrix(s);
        assert_eq!(first_part(&xs), 15);
    }

    #[test]
    fn second_part_works() {
        let s = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let xs = read_to_matrix(s);
        assert_eq!(second_part(&xs), 1134);
    }
}
