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

const MOD: i32 = 100_000;

// Tags: grid, counting
fn main() {
    input! {
        n: usize, m: usize,
    }
    let mut dp = vec![vec![[0; 4]; m]; n];
    dp[0][0] = [1, 1, 0, 0];
    for i in 0..n {
        for j in 0..m {
            for k in 0..2 {
                let (x, y) = if k == 0 {
                    if i > 0 {
                        (i - 1, j)
                    } else {
                        continue
                    }
                } else {
                    if j > 0 {
                        (i, j - 1)
                    } else {
                        continue
                    }
                };
                dp[i][j][2 + k]
                    = (dp[i][j][2 + k] + dp[x][y][k] + dp[x][y][2 + k]) % MOD;
                dp[i][j][k] = (dp[i][j][k] + dp[x][y][3 - k]) % MOD;
            }
        }
    }
    println!("{}", dp[n - 1][m - 1].iter().sum::<i32>() % MOD);
}
