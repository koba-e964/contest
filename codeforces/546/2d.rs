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
    input! {
        n: usize,
        m: usize,
        p: [usize1; n],
        uv: [(usize1, usize1); m],
    }
    let mut invp = vec![0; n];
    for i in 0..n {
        invp[p[i]] = i;
    }
    let mut g = vec![vec![]; n];
    let mut invg = vec![vec![]; n];
    let mut gh = vec![HashSet::new(); n];
    for &(u, v) in &uv {
        g[invp[u]].push(invp[v]);
        invg[invp[v]].push(invp[u]);
        gh[invp[u]].insert(invp[v]);
    }
    let mut cnt = 0;
    let mut pos = n - 1;
    let mut place: Vec<_> = (0..n).collect();
    invg[n - 1].sort();
    for &v in invg[n - 1].iter().rev() {
        let mut ok = true;
        for j in v..pos {
            assert_eq!(place[j], v);
            if gh[v].contains(&place[j + 1]) {
                // swap
                place.swap(j, j + 1);
            } else {
                ok = false;
                break;
            }
        }
        if ok {
            cnt += 1;
            pos -= 1;
        }
    }
    puts!("{}\n", cnt);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
