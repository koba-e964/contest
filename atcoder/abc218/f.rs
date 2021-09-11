use std::cmp::*;
use std::collections::*;
use std::io::{Write, BufWriter};
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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

const INF: i32 = 100_000;

fn calc(n: usize, st: &[(usize, usize)]) -> Vec<Vec<i32>> {
    let m = st.len();
    let mut ans = vec![vec![INF; n]; m + 1];
    ans[0][0] = 0;
    let mut que = VecDeque::new();
    let mut g = vec![vec![]; n];
    for i in 0..m {
        let (s, t) = st[i];
        for j in 0..n {
            ans[i + 1][j] = ans[i][j];
        }
        que.push_back((ans[i + 1][s] + 1, t));
        while let Some((d, v)) = que.pop_front() {
            if ans[i + 1][v] <= d {
                continue;
            }
            ans[i + 1][v] = d;
            for &w in &g[v] {
                if ans[i + 1][w] > d + 1 {
                    que.push_back((d + 1, w));
                }
            }
        }
        g[s].push(t);
    }
    ans
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        st: [(usize1, usize1); m],
    }
    let dp = calc(n, &st);
    let mut ts = vec![(0, 0); m];
    for i in 0..m {
        let (s, t) = st[i];
        ts[m - 1 - i] = (n - t - 1, n - s - 1);
    }
    let mut ep = calc(n, &ts);
    ep.reverse();
    for v in &mut ep {
        v.reverse();
    }
    for i in 0..m {
        let mut mi = INF;
        for j in 0..n {
            mi.chmin(dp[i][j] + ep[i + 1][j]);
        }
        puts!("{}\n", if mi >= INF { -1 } else { mi });
    }
}
