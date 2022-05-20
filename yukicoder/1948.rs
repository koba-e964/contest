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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    input! {
        h: usize, w: usize,
        a: [[i64; w]; h],
    }
    let mut dp = vec![vec![[0; 2]; w]; h];
    dp[0][0] = [a[0][0]; 2];
    for i in 0..h {
        for j in 0..w {
            for b in 0..2 {
                let val = dp[i][j][b];
                if i < h - 1 {
                    let x = a[i + 1][j];
                    if val > x {
                        dp[i + 1][j][b].chmax(val + x);
                    } else if b == 0 {
                        dp[i + 1][j][1].chmax(val);
                    }
                }
                if j < w - 1 {
                    let x = a[i][j + 1];
                    if val > x {
                        dp[i][j + 1][b].chmax(val + x);
                    } else if b == 0 {
                        dp[i][j + 1][1].chmax(val);
                    }
                }
            }
        }
    }
    println!("{}", if dp[h - 1][w - 1][1] > a[h - 1][w - 1] {
        "Yes"
    } else {
        "No"
    });
}
