///最长回文子串
fn main() {
    let s = "babad".to_string();
    println!("{}", longest_palindrome(s));
}

fn longest_palindrome(s: String) -> String {
    let s = s.chars().collect::<Vec<char>>();
    let n = s.len();
    let mut dp = vec![vec![false; n]; n];
    for i in 0..n {
        dp[i][i] = true;
    }

    let mut res = String::new();
    for i in (0..n).rev() {
        for j in i..n {
            if s[i] == s[j] && (j - i <= 2 || dp[i + 1][j - 1]) {
                dp[i][j] = true;
                if res.len() < j - i + 1 {
                    res = s[i..=j].iter().collect();
                    return res;
                }
            }
        }
    }

}