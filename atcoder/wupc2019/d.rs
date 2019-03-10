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

fn op(len: usize, a: &mut [usize], count: &mut usize, idx: usize, newval: usize) {
    if a[idx] == len {
        *count -= 1;
    }
    a[idx] = newval;
    if newval == len {
        *count += 1;
    }
}

fn sweep(g: &[Vec<usize>], len: usize) -> Option<usize> {
    let n = g.len();
    // sweep
    let mut freq = vec![0; n];
    let mut cnt = 0;
    for i in 0..len {
        for &v in &g[i] { 
            let val = freq[v] + 1;
            op(len, &mut freq, &mut cnt, v, val);
        }
    }
    for i in 0..n - len + 1 {
        if cnt == 0 {
            return Some(i);
        }
        // progress
        if i < n - len {
            for &v in &g[i + len] {
                let val = freq[v] + 1;
                    op(len, &mut freq, &mut cnt, v, val);
                }
            for &v in &g[i] {
                let val = freq[v] - 1;
                op(len, &mut freq, &mut cnt, v, val);
            }
        }
    }
    None
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
        ab: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[b].push(a);
    }
    for i in 0..n {
        g[i].push(i);
    }
    let mut pass = n + 1;
    let mut fail = 1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        if sweep(&g, mid).is_some() {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    // len = pass
    if pass > n {
        puts!("-1\n");
        return;
    }
    let l = sweep(&g, pass).unwrap();
    puts!("{} {}\n", l + 1, l + pass);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
