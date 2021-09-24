// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input!(a: usize, b: usize, c: usize, n: usize);
    let mut dp = vec![vec![vec![vec![0.0; c + 1]; b + 1]; a + 1]; n + 1];
    dp[0][a][b][c] = 1.0;
    for i in 0..n {
        for j in 1..a + 1 {
            for k in 1..b + 1 {
                for l in 1..c + 1 {
                    let x = (j + k + l) as f64;
                    let me = dp[i][j][k][l];
                    let whole = x * (x - 1.0);
                    let mut rem = whole;
                    dp[i + 1][j - 1][k][l] += me * (j * (j - 1)) as f64 / whole;
                    dp[i + 1][j][k - 1][l] += me * (k * (k - 1)) as f64 / whole;
                    dp[i + 1][j][k][l - 1] += me * (l * (l - 1)) as f64 / whole;
                    rem -= (j * (j - 1)) as f64;
                    rem -= (k * (k - 1)) as f64;
                    rem -= (l * (l - 1)) as f64;
                    dp[i + 1][j][k][l] += me * rem / whole;
                }
            }
        }
    }
    let mut ans = [0.0; 3];
    for j in 0..a + 1 {
        for k in 0..b + 1 {
            for l in 0..c + 1 {
                let p = dp[n][j][k][l];
                ans[0] += (a - j) as f64 * p;
                ans[1] += (b - k) as f64 * p;
                ans[2] += (c - l) as f64 * p;
            }
        }
    }
    println!("{} {} {}", ans[0], ans[1], ans[2]);
}
