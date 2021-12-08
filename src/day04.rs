use std::collections::HashSet;

pub fn first_part(seq: &Vec<i32>, xxs: &Vec<Vec<Vec<i32>>>) -> i32 {
    let n = xxs.len();
    let (row, col) = (xxs[0].len(), xxs[0][0].len());
    let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; row]; col]; n];
    for &v in seq {
        for (idx, matrix) in xxs.iter().enumerate() {
            'outer: for i in 0..row {
                for j in 0..col {
                    if matrix[i][j] == v {
                        visited[idx][i][j] = true;
                        if complete_board(&visited[idx]) {
                            return calculate_board(&visited[idx], matrix, matrix[i][j]);
                        }
                        break 'outer;
                    }
                }
            }
        }
    }
    -1
}

pub fn second_part(seq: &Vec<i32>, xxs: &Vec<Vec<Vec<i32>>>) -> i32 {
    let n = xxs.len();
    let (row, col) = (xxs[0].len(), xxs[0][0].len());
    let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; row]; col]; n];
    let mut completed_boards = HashSet::new();
    for &v in seq {
        for (idx, matrix) in xxs.iter().enumerate() {
            'outer: for i in 0..row {
                for j in 0..col {
                    if matrix[i][j] == v {
                        visited[idx][i][j] = true;
                        if complete_board(&visited[idx]) {
                            completed_boards.insert(idx);
                            if completed_boards.len() == n {
                                return calculate_board(&visited[idx], matrix, matrix[i][j]);
                            }
                        }
                        break 'outer;
                    }
                }
            }
        }
    }
    -1
}

fn calculate_board(visited: &Vec<Vec<bool>>, matrix: &Vec<Vec<i32>>, v: i32) -> i32 {
    let (row, col) = (matrix.len(), matrix[0].len());
    let mut unmarked_sum = 0;
    for i in 0..row {
        for j in 0..col {
            if !visited[i][j] {
                unmarked_sum += matrix[i][j];
            }
        }
    }
    unmarked_sum * v
}

fn complete_board(board: &Vec<Vec<bool>>) -> bool {
    let any_row_complete = board.iter().any(|row| row.iter().all(|&e| e));
    any_row_complete || {
        let (m, n) = (board.len(), board[0].len());
        for j in 0..n {
            let mut flag = true;
            for i in 0..m {
                flag &= board[i][j];
            }
            if flag {
                return true;
            }
        }
        false
    }
}

pub fn parse_day04(mut xs: Vec<&str>) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
    let seq: Vec<i32> = xs
        .remove(0)
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let mut xxs: Vec<Vec<Vec<i32>>> = Vec::with_capacity(seq.len());
    for ss in xs.chunks(6) {
        let mut matrix: Vec<Vec<i32>> = vec![];
        for &s in ss.iter().skip(1) {
            matrix.push(
                s.split_ascii_whitespace()
                    .map(|e| e.parse::<i32>().unwrap())
                    .collect(),
            );
        }
        xxs.push(matrix);
    }
    (seq, xxs)
}

#[cfg(test)]
mod tests {
    use crate::day04::parse_day04;

    use super::first_part;
    use super::second_part;

    #[test]
    fn first_part_works() {
        let s = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let (seq, xxs) = parse_day04(s.split('\n').collect());
        assert_eq!(first_part(&seq, &xxs), 4512);
    }

    #[test]
    fn second_part_works() {
        let s = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let (seq, xxs) = parse_day04(s.split('\n').collect());
        assert_eq!(second_part(&seq, &xxs), 1924);
    }
}
