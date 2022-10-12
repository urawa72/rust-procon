pub struct Solution;

#[allow(clippy::needless_range_loop)]
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut v = [0i8; 9];
            for j in 0..9 {
                let num = board[i][j];
                if num == '.' {
                    continue;
                }
                let num = (num as i32 - 48) as usize - 1;
                v[num] += 1;
                if 1 < v[num] {
                    return false;
                }
            }
        }

        for i in 0..9 {
            let mut v = [0i8; 9];
            for j in 0..9 {
                let num = board[j][i];
                if num == '.' {
                    continue;
                }
                let num = (num as i32 - 48) as usize - 1;
                v[num] += 1;
                if 1 < v[num] {
                    return false;
                }
            }
        }

        for i in 0..3 {
            let ii = i * 3;
            let mut v = [0i8; 9];
            for j in 0..3 {
                for k in ii..ii + 3 {
                    let num = board[j][k];
                    if num == '.' {
                        continue;
                    }
                    let num = (num as i32 - 48) as usize - 1;
                    v[num] += 1;
                    if 1 < v[num] {
                        return false;
                    }
                }
            }
            let mut v = [0i8; 9];
            for j in 3..6 {
                for k in ii..ii + 3 {
                    let num = board[j][k];
                    if num == '.' {
                        continue;
                    }
                    let num = (num as i32 - 48) as usize - 1;
                    v[num] += 1;
                    if 1 < v[num] {
                        return false;
                    }
                }
            }
            let mut v = [0i8; 9];
            for j in 6..9 {
                for k in ii..ii + 3 {
                    let num = board[j][k];
                    if num == '.' {
                        continue;
                    }
                    let num = (num as i32 - 48) as usize - 1;
                    v[num] += 1;
                    if 1 < v[num] {
                        return false;
                    }
                }
            }
        }

        true
    }
}
