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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    macro_rules! fail {
        () => (puts!("No\n"); return);
    }
    input! {
        n: usize,
        s: chars,
        a: i64, b: i64, c: i64, d: i64,
    }
    let mut e = [0; 2];
    e[0] = a + b + c;
    e[1] = a + b + d;
    let mut act = [0; 2];
    for i in 0..n {
        if s[i] == 'o' {
            act[0] += 1;
        } else {
            act[1] += 1;
        }
    }
    if act != e {
        fail!();
    }
    let mut lens = vec![];
    let mut cur = s[0];
    let mut cnt = 1;
    for i in 1..n {
        if cur != s[i] {
            cnt += 1;
            cur = s[i];
        } else {
            lens.push((cnt, cur));
            cnt = 1;
        }
    }
    lens.push((cnt, cur));
    let mut even_a = vec![];
    let mut even_b = vec![];
    let mut odd = 0;
    for (l, t) in lens {
        if l % 2 == 1 {
            odd += l / 2;
        } else if t == 'o' {
            even_b.push(l / 2 - 1);
        } else {
            even_a.push(l / 2 - 1);
        }
    }
    even_a.sort();
    even_b.sort();
    let mut ea_acc = vec![0; even_a.len() + 1];
    let mut eb_acc = vec![0; even_b.len() + 1];
    for i in 0..even_a.len() {
        ea_acc[i + 1] = ea_acc[i] + even_a[i];
    }
    for i in 0..even_b.len() {
        eb_acc[i + 1] = eb_acc[i] + even_b[i];
    }
    for i in 0..even_a.len() + 1 {
        let ac = min(a, i as i64 + ea_acc[i]);
        let mut sumall = ea_acc[even_a.len()] - ea_acc[i]
            + eb_acc[even_b.len()] + odd;
        let diff = max(0, a + b - (ac + sumall));
        //eprintln!("i = {}, diff = {}", i, diff);
        if diff > even_b.len() as i64 { continue; }
        let bc = min(b, diff + eb_acc[diff as usize]);
        sumall -= eb_acc[diff as usize];
        if ac + bc + sumall >= a + b {
            puts!("Yes\n");
            return;
        }
    }
    fail!();
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
