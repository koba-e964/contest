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

const INF: i64 = 1 << 40;

fn dfs(v: usize, par: usize, g: &[Vec<usize>], vs: &[i64]) -> [(i64, i64); 3] {
    let mut ret = [(INF, INF); 3];
    if vs[v] > 0 {
        ret[0] = (0, 1);
        ret[1] = (0, 0);
    } else {
        ret[0] = (0, 0);
    }
    for &w in &g[v] {
        if par == w { continue; }
        let mut sub = dfs(w, v, g, vs);
        for i in 0..3 {
            if sub[i].0 < INF {
                sub[i].0 += sub[i].1;
            }
        }
        let mut nxt = [(INF, INF); 3];
        for i in 0..3 {
            if ret[i].0 >= INF { continue; }
            for j in 0..3 - i {
                if sub[j].0 >= INF { continue; }
                let sum = ret[i].0 + sub[j].0;
                let mut rem = ret[i].1 + sub[j].1;
                rem %= 2;
                nxt[i + j].0 = min(nxt[i + j].0, sum);
                nxt[i + j].1 = rem;
            }
        }
        ret = nxt;
    }
    ret
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
        cd: [(usize1, usize1); n - 1],
    }
    let mut cddeg = vec![0i32; n];
    for &(c, d) in &cd {
        cddeg[c] += 1;
        cddeg[d] += 1;
    }
    let mut vs = vec![0; n];
    for i in 0..n {
        for _ in 0..cddeg[i] % 2 {
            vs[i] += 1;
        }
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let tot = dfs(0, n, &g, &vs);
    puts!("{}\n",  tot[2].0 + n as i64 - 1);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
