fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

// Solved with hints
// <https://chatgpt.com/share/69189278-b378-8010-9ae6-621b782c730d>
fn main() {
    getline();
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let n = a.len();
    let mut dp = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    for s in (0..n).rev() {
        for l in 0..=n - s {
            let r = l + s;
            for c in 0..n + 1 {
                if c >= 1 {
                    if l >= 1 {
                        dp[l][r][c] = dp[l][r][c].max(dp[l - 1][r][c - 1] + a[l - 1]);
                    }
                    if r < n {
                        dp[l][r][c] = dp[l][r][c].max(dp[l][r + 1][c - 1] + a[r]);
                    }
                }
                if c < n {
                    if r < n && (l == 0 || a[l - 1] < a[r]) {
                        dp[l][r][c] = dp[l][r][c].max(dp[l][r + 1][c + 1]);
                    }
                    if l > 0 && (r == n || a[l - 1] > a[r]) {
                        dp[l][r][c] = dp[l][r][c].max(dp[l - 1][r][c + 1]);
                    }
                }
            }
        }
    }
    for i in 0..n + 1 {
        println!("{}", dp[i][i][1]);
    }
}
