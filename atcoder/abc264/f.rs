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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn calc(r: &[i64], c: &[i64], a: &[Vec<i32>]) -> i64 {
    let h = r.len();
    let w = c.len();
    const INF: i64 = 1 << 50;
    let mut dp = vec![vec![[[INF; 2]; 2]; w]; h];
    dp[0][0] = [[0, c[0]], [r[0], r[0] + c[0]]];
    for x in 0..2 {
        for y in 0..2 {
            if (x ^ y) as i32 != a[0][0] {
                dp[0][0][x][y] = INF;
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if i + j == 0 { continue; }
            let mut me = [[INF; 2]; 2];
            for x in 0..2 {
                for y in 0..2 {
                    if i > 0 {
                        let z = a[i][j] as usize ^ y;
                        me[z][y] = min(me[z][y], dp[i - 1][j][x][y] + r[i] * z as i64);
                    }
                    if j > 0 {
                        let z = a[i][j] as usize ^ x;
                        me[x][z] = min(me[x][z], dp[i][j - 1][x][y] + c[j] * z as i64);
                    }
                }
            }
            dp[i][j] = me;
        }
    }
    let mut ans = dp[h - 1][w - 1][0][0];
    for x in 0..2 {
        for y in 0..2 {
            ans = min(ans, dp[h - 1][w - 1][x][y]);
        }
    }
    ans
}

fn main() {
    input! {
        h: usize, w: usize,
        r: [i64; h],
        c: [i64; w],
        a: [chars; h],
    }
    let a: Vec<Vec<i32>> = a.into_iter().map(|v| v.into_iter().map(|c| (c as u8 - b'0') as _).collect()).collect();
    let mut mi = calc(&r, &c, &a);
    let mut a = a;
    for i in 0..h {
        for j in 0..w {
            a[i][j] ^= 1;
        }
    }
    mi = min(mi, calc(&r, &c, &a));
    println!("{}", mi);
}
