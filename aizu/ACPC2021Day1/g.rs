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

const INF: i64 = 1 << 50;

fn main() {
    input! {
        n: usize,
        e: [[i64; n]; n],
    }
    let mut dp = vec![vec![(INF, 0, 0); n]; n];
    dp[0][0] = (0, 0, 0);
    for i in 0..n - 1 {
        for j in 0..n - 1 {
            let val = dp[i][j].0;
            let to = max(i, j) + 1;
            dp[to][j].chmin((val + e[i][to], i, j));
            dp[i][to].chmin((val + e[to][j], i, j));
            dp[to][to].chmin((val + e[i][to] + e[to][j], i, j));
        }
    }
    println!("{}", dp[n - 1][n - 1].0);
    let mut path0 = vec![];
    let mut path1 = vec![];
    let mut cur = (n - 1, n - 1);
    while cur != (0, 0) {
        let (x, y) = cur;
        cur = (dp[x][y].1, dp[x][y].2);
        let (px, py) = cur;
        if px < x {
            path0.push(px);
        }
        if py < y {
            path1.push(py);
        }
    }
    path0.reverse();
    path0.push(n - 1);
    path0.extend(&path1);
    println!("{}", path0.len());
    for i in 0..path0.len() {
        print!("{}{}", path0[i] + 1, if i + 1 == path0.len() { "\n" } else { " " });
    }
}
