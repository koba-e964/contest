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
        n: usize,
        a: [usize1; n],
        b: [usize1; n],
    }
    let mut occ_a = vec![0; n];
    let mut occ_b = vec![0; n];
    for &a in &a {
        occ_a[a] += 1;
    }
    for &b in &b {
        occ_b[b] += 1;
    }
    for i in 0..n {
        if occ_a[i] + occ_b[i] > n {
            puts!("No\n");
            return;
        }
    }
    let mut div = 0;
    let mut sum = 0;
    while sum < n {
        sum += occ_a[div] + occ_b[div];
        div += 1;
    }
    let inf = 1usize << 20;
    let mut ans = vec![inf; n];
    let mut asum = 0;
    for i in 0..div {
        asum += occ_a[i];
    }
    let mut pos = asum;
    let mut fst = occ_b[..div].to_vec();
    let mut lim = vec![0; div];
    for i in 0..div {
        lim[i] = asum - occ_a[i];
    }
    let mut upper = n;
    for i in 0..div {
        let r = fst[i] - min(lim[i], fst[i]);
        fst[i] -= r;
        for _ in 0..r {
            upper -= 1;
            ans[upper] = i;
        }
    }
    for i in 0..div {
        if i == 1 && pos < asum {
            pos = 0;
        }
        for _ in 0..fst[i] {
            if pos >= upper {
                pos = 0;
                if i == 0 {
                    pos = occ_a[0];
                }
            }
            while ans[pos] != inf {
                pos += 1;
            }
            ans[pos] = i;
            pos += 1;
        }
    }
    for i in 0..n {
        assert!(i < asum || ans[i] != inf);
    }
    pos = 0;
    for i in div..n {
        for _ in 0..occ_b[i] {
            while ans[pos] != inf {
                pos += 1;
            }
            ans[pos] = i;
            pos += 1;
        }
    }
    puts!("Yes\n");
    for i in 0..n {
        puts!("{}{}", ans[i] + 1, if i + 1 == n { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
