pub fn first_part(xs: Vec<Vec<u8>>) -> i32 {
    let mut board = xs;
    let (m, n) = (board.len(), board[0].len());
    let mut step = 0;

    loop {
        // debug(&board);
        let mut cant_move_flag = true;
        let mut next_board = board.clone();
        for i in 0..m {
            for j in 0..n {
                let nexti = i;
                let nextj = (j + 1) % n;
                if board[i][j] == b'>' && board[nexti][nextj] == b'.' {
                    next_board[nexti][nextj] = b'>';
                    next_board[i][j] = b'.';
                    cant_move_flag = false;
                }
            }
        }
        board = next_board.clone();
        for i in 0..m {
            for j in 0..n {
                let nexti = (i + 1) % m;
                let nextj = j;
                if board[i][j] == b'v' && board[nexti][nextj] == b'.' {
                    next_board[nexti][nextj] = b'v';
                    next_board[i][j] = b'.';
                    cant_move_flag = false;
                }

            }
        }
        step += 1;
        if cant_move_flag {
            break;
        }
        board = next_board;
    }
    step
}

pub fn read_to_u8_matrix(s: &str) -> Vec<Vec<u8>> {
    let xs: Vec<&str> = s.lines().collect();
    let (m, n) = (xs.len(), xs[0].len());
    let mut matrix = vec![vec![0; n]; m];
    for i in 0..m {
        for (j, &ch) in xs[i].as_bytes().iter().enumerate() {
            matrix[i][j] = ch;
        }
    }
    matrix
}

fn debug(board: &Vec<Vec<u8>>) {
    for row in board {
        println!("{:?}", String::from_utf8_lossy(row));
    }
    println!("==========================");
}

#[cfg(test)]
mod tests {
    use super::first_part;
    use crate::day25::read_to_u8_matrix;

    #[test]
    fn first_part_works() {
        let s = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        println!("{:?}", first_part(read_to_u8_matrix(s)));
    }
}
