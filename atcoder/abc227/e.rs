use std::cmp::*;
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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        s: chars,
        k: usize,
    }
    let n = s.len();
    let mut a = vec![vec![]; 3];
    for i in 0..n {
        let idx = ['K', 'E', 'Y'].iter().position(|&x| x == s[i]).unwrap();
        a[idx].push(i);
    }
    let x = a[0].len();
    let y = a[1].len();
    let z = a[2].len();
    let lim = n * (n - 1) / 2 + 1;
    let mut dp = vec![vec![vec![vec![0i64; lim]; z + 1]; y + 1]; x + 1];
    dp[0][0][0][0] = 1;
    for i in 0..x + 1 {
        for j in 0..y + 1 {
            for l in 0..z + 1 {
                if i < x {
                    let idx = a[0][i];
                    let mut inv = 0;
                    for b in 0..j {
                        if a[1][b] > idx { inv += 1; }
                    }
                    for b in 0..l {
                        if a[2][b] > idx { inv += 1; }
                    }
                    for u in inv..lim {
                        dp[i + 1][j][l][u] += dp[i][j][l][u - inv];
                    }
                }
                if j < y {
                    let idx = a[1][j];
                    let mut inv = 0;
                    for b in 0..i {
                        if a[0][b] > idx { inv += 1; }
                    }
                    for b in 0..l {
                        if a[2][b] > idx { inv += 1; }
                    }
                    for u in inv..lim {
                        dp[i][j + 1][l][u] += dp[i][j][l][u - inv];
                    }
                }
                if l < z {
                    let idx = a[2][l];
                    let mut inv = 0;
                    for b in 0..i {
                        if a[0][b] > idx { inv += 1; }
                    }
                    for b in 0..j {
                        if a[1][b] > idx { inv += 1; }
                    }
                    for u in inv..lim {
                        dp[i][j][l + 1][u] += dp[i][j][l][u - inv];
                    }
                }
            }
        }
    }
    let mut ans = 0i64;
    for i in 0..min(lim, k + 1) {
        ans += dp[x][y][z][i];
    }
    println!("{}", ans);
}
