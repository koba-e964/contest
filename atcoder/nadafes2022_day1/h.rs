use std::cmp::*;
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
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

struct LCA {
    st: Vec<usize>,
    par: Vec<usize>,
    jmp: Vec<usize>,
    dep: Vec<usize>,
}

// Constant-factor speedup used in https://codeforces.com/contest/1083/submission/46874242.
// Base on HL-decomposition.
// par[root] = root should hold.
// Verified by https://codeforces.com/contest/1083/submission/51934575.
impl LCA {
    // For each node, make the most heavy child the first child.
    fn dfs_left(ch: &mut [Vec<usize>], v: usize, sz: &mut [usize],
                dep: &mut [usize], d: usize) {
        dep[v] = d;
        let mut s = 1;
        for i in 0..ch[v].len() {
            let w = ch[v][i];
            Self::dfs_left(ch, w, sz, dep, d + 1);
            s += sz[w];
            if sz[w] > sz[ch[v][0]] {
                ch[v].swap(i, 0);
            }
        }
        sz[v] = s;
    }
    fn dfs(ch: &[Vec<usize>], st: &mut [usize], v: usize,
           cnt: &mut usize, jmp: &mut [usize]) {
        st[v] = *cnt;
        *cnt += 1;
        if ch[v].len() >= 1 {
            jmp[ch[v][0]] = jmp[v];
        }
        for &w in &ch[v] {
            Self::dfs(ch, st, w, cnt, jmp);
        }
    }
    fn new(ch: &mut [Vec<usize>], par: &[usize], root: usize) -> Self {
        let n = ch.len();
        let mut st = vec![0; n];
        let mut cnt = 0;
        let mut sz = vec![0; n];
        let mut jmp = vec![0; n];
        let mut dep = vec![0; n];
        Self::dfs_left(ch, root, &mut sz, &mut dep, 0);
        for i in 0..n {
            jmp[i] = i;
        }
        Self::dfs(ch, &mut st, root, &mut cnt, &mut jmp);
        LCA {
            st: st,
            par: par.to_vec(),
            jmp: jmp,
            dep: dep,
        }
    }
    fn lca(&self, mut x: usize, mut y: usize) -> usize {
        let jmp = &self.jmp;
        let st = &self.st;
        while jmp[x] != jmp[y] {
            if st[x] < st[y] {
                std::mem::swap(&mut x, &mut y);
            }
            x = self.par[jmp[x]];
        }
        if st[x] < st[y] {
            x
        } else {
            y
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, par: usize, p: &mut [usize], ch: &mut [Vec<usize>], g: &[Vec<usize>]) {
    p[v] = par;
    for &w in &g[v] {
        if w == par { continue; }
        dfs(w, v, p, ch, g);
        ch[v].push(w);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        uv: [(usize1, usize1); n - 1],
        a: [[usize1]; q],
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut par = vec![0; n];
    let mut ch = vec![vec![]; n];
    dfs(0, n, &mut par, &mut ch, &g);
    let lca = LCA::new(&mut ch, &par, 0);
    for mut a in a {
        let k = a.len();
        a.sort_by_key(|&v| lca.st[v]);
        // perimeter
        let mut tot = 0;
        for i in 0..k {
            let x = a[i];
            let y = a[(i + 1) % k];
            let l = lca.lca(x, y);
            tot += lca.dep[x] + lca.dep[y] - 2 * lca.dep[l];
        }
        // diameter
        let mut ma = (0, 0);
        for &b in &a {
            let x = a[0];
            let y = b;
            let l = lca.lca(x, y);
            let dist = lca.dep[x] + lca.dep[y] - 2 * lca.dep[l];
            ma = max(ma, (dist, b));
        }
        let c = ma.1;
        for &b in &a {
            let x = c;
            let y = b;
            let l = lca.lca(x, y);
            let dist = lca.dep[x] + lca.dep[y] - 2 * lca.dep[l];
            ma = max(ma, (dist, b));
        }
        tot -= ma.0;
        puts!("{}\n", tot);
    }
}
