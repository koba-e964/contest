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

const INF: i32 = 1 << 25;

fn dfs(g: &[Vec<usize>], v: usize, par: usize) -> [i32; 2] {
    let mut ch = vec![];
    for &w in &g[v] {
        if par == w { continue; }
        ch.push(w);
    }
    if ch.len() == 0 {
        return [0, 1];
    }
    let mut ret = [1; 2];
    let mut ma = 0;
    for &w in &ch {
        let sub = dfs(g, w, v);
        for j in 0..2 {
            ret[j] += sub[j];
            if sub[j] == -INF {
                ret[j] = -INF;
            }
        }
        ma = max(ma, sub[1] - sub[0]);
    }
    if ch.len() >= 2 {
        ret[1] = -INF;
        ret[0] += ma;
    } else {
    }
    //eprintln!("dfs({}) = {:?}", v, ret);
    ret
}

// 0: parent doesn't need to have antennas
// 1: parent needs to have antennas
fn dfs2(g: &[Vec<usize>], v: usize, par: usize) -> [i32; 2] {
    let mut ch = vec![];
    for &w in &g[v] {
        if par == w { continue; }
        ch.push(w);
    }
    if ch.len() == 0 {
        return [0, 0];
    }
    if ch.len() == 1 {
        let mut sub = dfs2(g, ch[0], v);
        sub[0] = min(max(sub[0], 1), sub[1] + 1);
        return sub;
    }
    let mut ans = [0; 2];
    let mut vecs = vec![vec![]; 2];
    for &w in &ch {
        let sub = dfs2(g, w, v);
        for i in 0..2 {
            vecs[i].push(sub[1]);
        }
    }
    for i in 0..2 {
        vecs[i].sort();
        for j in i..vecs[i].len() {
            vecs[i][j] = max(vecs[i][j], 1);
        }
        for &a in &vecs[i] {
            ans[i] += a;
        }
    }
    debugln!("{} -> {:?}", v, ans);
    ans
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let ans;
    let mut sol = vec![0; n];
    /*
    for r in 0..n {
        let sub = dfs2(&g, r, n);
        sol[r] = if g[r].len() == 1 { sub[0] } else { sub[0] };
    }
     */
    let mut r = n;
    for i in 0..n {
        if g[i].len() == 1 {
            r = i;
            break;
        }
    }
    ans = dfs2(&g, r, n)[0];
    puts!("{}\n", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
