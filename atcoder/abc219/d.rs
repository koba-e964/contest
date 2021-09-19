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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
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
        n: usize,
        x: usize, y: usize,
        ab: [(usize, usize); n],
    }
    const INF: i32 = 1 << 28;
    let mut dp = vec![vec![INF; y + 1]; x + 1];
    dp[0][0] = 0;
    for &(a, b) in &ab {
        for i in (0..x + 1).rev() {
            for j in (0..y + 1).rev() {
                let ni = min(i + a, x);
                let nj = min(j + b, y);
                if (i, j) == (ni, nj) { continue; }
                let val = dp[i][j] + 1;
                dp[ni][nj].chmin(val);
            }
        }
    }
    println!("{}", if dp[x][y] >= INF { -1 } else { dp[x][y] });
}
