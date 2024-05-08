fn main() {
    let mut b = vec![vec!['X', 'X', 'X', 'X'],vec!['X', 'O', 'O', 'X'],vec!['X', 'X', 'O', 'X'],vec!['X', 'O', 'X', 'X' ]];
    solve(&mut b);
    println!("{:?}",b);
}

// 130. Surrounded Regions
pub fn solve(board: &mut Vec<Vec<char>>) {
    if board.is_empty() {
        return;
    }
    let m = board.len();
    let n = board[0].len();
    
    for i in 0..m {
        dfs(board, i, 0);
        dfs(board, i, n - 1);
    }
    
    for j in 0..n {
        dfs(board, 0, j);
        dfs(board, m - 1, j);     
    }

    for i in 0..m {
        for j in 0..n {
            if board[i][j] == 'O' {
                board[i][j] = 'X';
            } else if board[i][j] == 'A' {
                board[i][j] = 'O';
            }   
        }
    }
}

fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
    let m = board.len();
    let n = board[0].len();
    if i >= m || j >= n || i < 0 || j < 0 {
        return;
    }
    if board[i][j] != 'O' {
        return;
    }   

    board[i][j] = 'A';
    dfs(board, i + 1, j);
    dfs(board, i - 1, j);
    dfs(board, i, j + 1);
    dfs(board, i, j - 1);
}