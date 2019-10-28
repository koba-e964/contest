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
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
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
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
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
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        m: usize,
        st: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for &(s, t) in &st {
        g[s].push(t);
    }
    let mut dp1 = vec![0.0; n];
    for i in (0..n - 1).rev() {
        let mut sum = 0.0;
        let mut cnt = 0.0;
        for &t in &g[i] {
            sum += dp1[t];
            cnt += 1.0;
        }
        dp1[i] = sum / cnt + 1.0;
    }
    let mut dp2 = vec![0.0; n];
    let mut prob2 = vec![0.0; n];
    prob2[0] = 1.0;
    for i in 0..n - 1 {
        let cnt = g[i].len() as f64;
        for &t in &g[i] {
            prob2[t] += prob2[i] / cnt;
            dp2[t] += (dp2[i] + prob2[i]) / cnt;
        }
    }
    debugln!("dp1 = {:?}", dp1);
    debugln!("dp2 = {:?}", dp2);
    debugln!("prob2 = {:?}", prob2);
    let mut mi = 0.0;
    for i in 0..n {
        let mut sum = 0.0;
        let mut cnt = 0.0;
        for &t in &g[i] {
            sum += dp1[t];
            cnt += 1.0;
        }
        if cnt == 1.0 {
            continue;
        }
        for &t in &g[i] {
            let diff = (sum - dp1[t]) / (cnt - 1.0) - sum / cnt;
            let diff = diff * prob2[i];
            if mi > diff {
                mi = diff;
            }
        }
    }
    puts!("{}\n", mi + dp1[0]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
