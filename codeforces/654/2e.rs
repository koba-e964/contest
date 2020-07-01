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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, p: i64,
        a: [i64; n],
    }
    const W: i64 = 1_000_000_100;
    let mut a = a;
    a.sort(); a.reverse();
    let mut cand = vec![];
    let mut sup = 0;
    let mut inf = W;
    let mut banned = vec![0; p as usize];
    for i in 0..n {
        sup = max(sup, a[i] + i as i64 - n as i64);
        let r = a[i] + i as i64 - n as i64 % p + p;
        banned[(r % p) as usize] = max(banned[(r % p) as usize], a[i]);
        if (n as i64 - i as i64) % p == 0 {
            inf = min(inf, a[i]);
        }
    }
    if false {
        eprintln!("sup = {}, inf = {}", sup, inf);
        for i in 0..p as usize {
            eprintln!("banned[{}] = {:?}", i, banned[i]);
        }
    }
    for i in 0..p as usize {
        let banlim = banned[i as usize];
        let lo = max(sup, banlim + 1);
        let hi = inf;
        if lo > hi { continue; }
        let i = i as i64;
        let mut lo = (lo - i + p - 1) / p * p + i;
        while lo <= hi {
            cand.push(lo);
            lo += p;
        }
    }
    cand.sort();
    puts!("{}\n", cand.len());
    for i in 0..cand.len() {
        puts!("{}{}", cand[i], if i + 1 == cand.len() { "" } else { " " });
    }
    puts!("\n");
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
