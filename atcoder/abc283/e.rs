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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize,
        a: [[i32; w]; h],
    }
    const INF: i64 = 1 << 50;
    let mut res = vec![0; h];
    let mut dp = vec![[[INF; 2]; 2]; h];
    dp[0][0] = [0, 1];
    for i in 0..h {
        for j in 0..w {
            let mut skip = false;
            if j > 0 && a[i][j] == a[i][j - 1] {
                skip = true;
            }
            if j < w - 1 && a[i][j] == a[i][j + 1] {
                skip = true;
            }
            if skip { continue; }
            if i == 0 {
                let diff = a[i][j] ^ a[i + 1][j];
                if diff == 0 {
                    res[i] |= 1 << 2 | 1 << 3;
                } else {
                    res[i] |= 1 << 0 | 1 << 1;
                }
            } else if i == h - 1 {
                let diff = a[i][j] ^ a[i - 1][j];
                if diff == 0 {
                    res[i] |= 1 << 1 | 1 << 3;
                } else {
                    res[i] |= 1 << 0 | 1 << 2;
                }
            } else {
                let diff1 = a[i][j] ^ a[i + 1][j] ^ 1;
                let diff0 = a[i][j] ^ a[i - 1][j] ^ 1;
                let bit = diff1 << 1 | diff0;
                res[i] |= 1 << bit;
            }
        }
    }
    for i in 1..h {
        for b1 in 0..2 {
            for b2 in 0..2 {
                for b3 in 0..2 {
                    let diff1 = b3 ^ b2;
                    let diff0 = b1 ^ b2;
                    let bit = diff1 << 1 | diff0;
                    if (res[i - 1] & 1 << bit) != 0 { continue; }
                    dp[i][b2][b3] = min(dp[i][b2][b3], dp[i - 1][b1][b2] + b3 as i64);
                }
            }
        }
    }
    let mut mi = INF;
    for b1 in 0..2 {
        for b2 in 0..2 {
            let diff = b1 ^ b2;
            if (res[h - 1] & 1 << diff) != 0 { continue; }
            mi = min(mi, dp[h - 1][b1][b2]);
        }
    }
    println!("{}", if mi >= INF { -1 } else { mi });
}

