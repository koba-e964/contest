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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        a: usize, b: usize, c: usize,
        aa: [f64; a],
        bb: [f64; b],
    }
    const W: usize = 200;
    let mut fac = vec![0.0; W];
    fac[0] = 1.0;
    for i in 0..W - 1 {
        fac[i + 1] = fac[i] * (i as f64 + 1.0);
    }
    let mut ans = 0.0;
    let mut dp = vec![0.0; b + 1];
    let mut ep = vec![0.0; b + 1];
    dp[0] = 1.0;
    for i in 0..b {
        for j in 0..b + 1 {
            ep[j] = 0.0;
        }
        for j in 0..b {
            ep[j + 1] += dp[j] * bb[i];
            ep[j] += dp[j];
        }
        std::mem::swap(&mut dp, &mut ep);
    }
    for j in 0..b + 1 {
        for k in 0..b - j + 1 {
            // bbb...bb (abbb...bc) (b|c)*
            // C(b - j - k + c - 1, b - j - k, c - 1) / C(b, j) / C(1 + b + c, b, c)
            // (b - j - k + c - 1)! (b-j)! j! c! / ( (c-1)! (1 + b + c)! (b - j - k)!)
            let mut tmp = fac[b - j + c - k - 1];
            tmp *= fac[b - j];
            tmp *= fac[j];
            tmp *= fac[c];
            tmp /= fac[c - 1];
            tmp /= fac[1 + b + c];
            tmp /= fac[b - j - k];
            ans += tmp * dp[j];
            eprintln!("tmp = {}, dp[j] = {}", tmp, dp[j]);
        }
    }
    let mut sum = 0.0;
    for i in 0..a {
        sum += aa[i];
    }
    puts!("{}\n", ans * sum);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
