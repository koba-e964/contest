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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

// Tags: polynomial-complexity, polynomial-time
fn main() {
    input! {
        n: usize,
        s: [chars; n],
    }
    let mut a = vec![0i64; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                a[i] |= 1 << j;
            }
        }
    }
    let mut dp = vec![vec![vec![vec![0; n + 1]; n + 1]; n + 1]; n + 1];
    for s1 in 1..n + 1 {
        for s2 in 1..n + 1 {
            for i in 0..n - s1 + 1 {
                let j = i + s1;
                for k in 0..n - s2 + 1 {
                    let l = k + s2;
                    let mask = ((1 << s2) - 1) << k;
                    let mut me = max(s1, s2);
                    if (i..j).all(|p| (a[p] & mask) == 0) {
                        me = 0;
                    }
                    for x in i + 1..j {
                        me.chmin(
                            dp[i][x][k][l] + dp[x][j][k][l],
                        );
                    }
                    for x in k + 1..l {
                        me.chmin(
                            dp[i][j][k][x] + dp[i][j][x][l],
                        );
                    }
                    dp[i][j][k][l] = me;
                }
            }
        }
    }
    println!("{}", dp[0][n][0][n]);
}
