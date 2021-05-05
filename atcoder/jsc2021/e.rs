#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
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
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change {
    fn chmax(&mut self, x: Self);
    fn chmin(&mut self, x: Self);
}
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) {
        if *self < x { *self = x; }
    }
    fn chmin(&mut self, x: T) {
        if *self > x { *self = x; }
    }
}

const B: usize = 26;

fn dfs(mut k: usize, mut freq: Vec<[usize; B]>) -> (Vec<[usize; B]>, usize) {
    let mut sum = 0;
    while k > 0 {
        let m = freq.len();
        if m % 2 == 1 {
            sum += freq[m / 2].iter().min().unwrap();
        }
        let mut nxt = vec![[0; B]; m / 2];
        for i in 0..m / 2 {
            for j in 0..B {
                nxt[i][j] += freq[i][j];
                nxt[i][j] += freq[m - 1 - i][j];
            }
        }
        k -= 1;
        freq = nxt;
    }
    (freq, sum)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    macro_rules! fail {
        () => {
            puts!("impossible\n");
            return;
        }
    }
    input! {
        k: usize,
        s: chars,
    }
    const INF: usize = 1 << 30;
    let n = s.len();
    if k >= 30 || (k > 0 && 1 << (k - 1) > n)  {
        fail!();
    }
    let mut cost = vec![[1; 26]; n];
    for i in 0..n {
        let idx = (s[i] as u8 - b'a') as usize;
        cost[i][idx] = 0;
    }
    let (folded, mut rem) = dfs(k, cost);
    let m = folded.len();
    if m == 1 {
        fail!();
    }
    if m >= 2 {
        if m % 2 == 1 {
            rem += folded[m / 2].iter().min().unwrap();
        }
        let mut same = vec![INF; m / 2];
        let mut diff = vec![INF; m / 2];
        for i in 0..m / 2 {
            for j in 0..B {
                same[i].chmin(folded[i][j] + folded[m - 1 - i][j]);
            }
        }
        for i in 0..m / 2 {
            for j in 0..B {
                for k in 0..B {
                    if j != k {
                        diff[i].chmin(folded[i][j] + folded[m - 1 - i][k]);
                    }
                }
            }
        }
        let mut dp = vec![[INF; 2]; m / 2 + 1];
        dp[0][0] = 0;
        for i in 0..m / 2 {
            for j in 0..2 {
                let val = dp[i][j] + same[i];
                dp[i + 1][j].chmin(val);
                let val = dp[i][j] + diff[i];
                dp[i + 1][1].chmin(val);
            }
        }
        rem += dp[m / 2][1];
    }
    puts!("{}\n", rem);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
