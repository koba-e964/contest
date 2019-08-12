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
        k: [i64; n],
        go: [[usize1]; n],
        xy: [(usize1, i64)],
    }
    const W: usize = 2520;
    const WW: i64 = 2520;
    let mut nxt = vec![0; W * n];
    for i in 0..n {
        let m = go[i].len();
        for j in 0..W {
            let x = j as i64 + k[i];
            let vidx = (((x % WW) + WW) % WW) as usize;
            let idx = vidx % m;
            nxt[i * W + j] = go[i][idx] * W + vidx;
        }
    }
    let mut cnt = vec![0; W * n];
    let mut vis = vec![0; W * n];
    let mut col = vec![false; W * n];
    for i in 0..W * n {
        let mut path = vec![];
        let mut v = i;
        let mut a = 1;
        let per;
        let mut dist = HashSet::new();
        loop {
            if cnt[v] != 0 {
                per = cnt[v];
                break;
            }
            if vis[v] != 0 {
                col[v] = true;
                for j in vis[v] - 1..a - 1 {
                    let idx = path[j] / W;
                    dist.insert(idx);
                }
                per = dist.len();
                break;
            }
            path.push(v);
            vis[v] = a;
            v = nxt[v];
            a += 1;
        }
        while let Some(v) = path.pop() {
            cnt[v] = per;
        }
    }
    for &(x, y) in &xy {
        let y = ((y % WW + WW) % WW) as usize;
        puts!("{}\n", cnt[x * W + y]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
