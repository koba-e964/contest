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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

// 解説解。xor convolution
const B: usize = 18;

fn calc(mut cnt: Vec<i64>, x: usize) -> i64 {
    for i in 0..B {
        for b in 0..1 << B {
            if (b & 1 << i) != 0 {
                continue;
            }
            let x = cnt[b];
            let y = cnt[b | 1 << i];
            cnt[b] = x + y;
            cnt[b | 1 << i] = x - y;
        }
    }
    let cnt0 = cnt[0];
    for b in 0..1 << B {
        cnt[b] = cnt[b] * cnt[b];
    }
    for i in 0..B {
        for b in 0..1 << B {
            if (b & 1 << i) != 0 {
                continue;
            }
            let x = cnt[b];
            let y = cnt[b | 1 << i];
            cnt[b] = (x + y) / 2;
            cnt[b | 1 << i] = (x - y) / 2;
        }
    }
    let mut sum = 0;
    for i in 0..x {
        sum += cnt[i];
    }
    (sum - cnt0) / 2 
}
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, x: usize,
        a: [usize; n],
    }
    let mut cnt = vec![0i64; 1 << B];
    for &a in &a {
        cnt[a] += 1;
    }
    let all = calc(cnt.clone(), x);
    let mut sum = 0;
    for i in 0..B {
        let mut cp = cnt.clone();
        for bits in 0..1 << B {
            if (bits & 1 << i) != 0 {
                cp[bits] = 0;
            }
        }
        let del = calc(cp, x);
        sum += (all - del) << i;
    }
    puts!("{}\n", sum);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
