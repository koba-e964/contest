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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn dfs1(v: usize, par: usize, vis: &mut [bool], g: &[Vec<usize>]) -> Result<(), (usize, Vec<usize>)> {
    if vis[v] {
        return Err((v, vec![]));
    }
    vis[v] = true;
    for &w in &g[v] {
        if par == w {
            continue;
        }
        if let Err((ret, mut vec)) = dfs1(w, v, vis, g) {
            if ret == g.len() {
                return Err((ret, vec));
            }
            vec.push(v);
            return Err((if v == ret { g.len() } else { ret }, vec))
        }
    }
    Ok(())
}

fn dfs2(v: usize, par: usize, vis: &mut [bool], g: &[Vec<usize>]) -> i64 {
    if vis[v] {
        return 0;
    }
    vis[v] = true;
    let mut s = 1;
    for &w in &g[v] {
        if par == w {
            continue;
        }
        s += dfs2(w, v, vis, g);
    }
    s
}

// Tags: namori-graph, dfs
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
        ab: [[(usize1, usize1)]],
    }
    for ab in ab {
        let n = ab.len();
        let mut g = vec![vec![]; n];
        for &(a, b) in &ab {
            g[a].push(b);
            g[b].push(a);
        }
        let mut vis = vec![false; n];
        let (_, cyc) = dfs1(0, n, &mut vis, &g).unwrap_err();
        let mut vis = vec![false; n];
        for &c in &cyc {
            vis[c] = true;
        }
        let mut ans = n as i64 * (n - 1) as i64;
        for &c in &cyc {
            vis[c] = false;
            let s = dfs2(c, n, &mut vis, &g);
            ans -= s * (s - 1) / 2;
        }
        puts!("{}\n", ans);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
