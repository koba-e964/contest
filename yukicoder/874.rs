use std::cmp::*;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn compile(a: &[char]) -> Vec<(char, u8)> {
    let mut ans: Vec<(char, u8)> = vec![];
    for &c in a {
        let l = ans.len();
        if c == '?' {
            ans[l - 1].1 = 1;
        } else if c == '*' {
            ans[l - 1].1 = 2;
        } else {
            ans.push((c, 0));
        }
    }
    ans
}

fn main() {
    let a: Vec<_> = getline().trim().chars().collect();
    let b: Vec<_> = getline().trim().chars().collect();
    let a = compile(&a);
    let b = compile(&b);
    let n = a.len();
    let m = b.len();
    const INF: i64 = 1 << 40;
    let mut dp = vec![vec![INF; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n + 1 {
        let (xc, xk) = if i < n { a[i] } else { ('+', 0) };
        for j in 0..m + 1 {
            let val = dp[i][j];
            let (yc, yk) = if j < m { b[j] } else { ('-', 0) };
            if i < n && j < m {
                if xc == yc || (xk != 0 && yk != 0) {
                    dp[i + 1][j + 1] = min(dp[i + 1][j + 1], val);
                } else {
                    dp[i + 1][j + 1] = min(dp[i + 1][j + 1], val + 1);
                }
            }
            if j < m {
                let add = if yk != 0 || (xc == yc && xk == 2) {
                    0
                } else {
                    1
                };
                dp[i][j + 1] = min(dp[i][j + 1], val + add);
            }
            if i < n {
                let add = if xk != 0 || (xc == yc && yk == 2) {
                    0
                } else {
                    1
                };
                dp[i + 1][j] = min(dp[i + 1][j], val + add);
            }
        }
    }
    println!("{}", dp[n][m]);
}
