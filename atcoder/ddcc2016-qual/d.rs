#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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

trait Change {
    fn chmax(&mut self, x: Self);
    fn chmin(&mut self, x: Self);
}
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) {
        if *self < x {
            *self = x;
        }
    }
    fn chmin(&mut self, x: T) {
        if *self > x {
            *self = x;
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

fn centroid_decompose(g: &[Vec<(usize, i64)>]) -> Vec<(usize, usize, i64)> {
    fn find_subtree_sizes(
        g: &[Vec<(usize, i64)>],
        v: usize,
        par: usize,
        dp: &mut [usize],
        vis: &[bool],
    ) {
        let mut sum = 1;
        for &(w, _) in &g[v] {
            if par == w || vis[w] {
                continue;
            }
            find_subtree_sizes(g, w, v, dp, vis);
            sum += dp[w];
        }
        dp[v] = sum;
    }
    fn centroid_decompose_inner(
        g: &[Vec<(usize, i64)>],
        v: usize,
        par: usize,
        cost: i64,
        edges: &mut Vec<(usize, usize, i64)>,
        dp: &mut [usize],
        vis: &mut [bool],
    ) {
        let n = g.len();
        find_subtree_sizes(g, v, n, dp, vis);
        let (cent, dist) = {
            let sz = dp[v];
            let find_centroid = |mut v: usize, mut par: usize| {
                let mut dist = 0;
                loop {
                    let mut has_majority = false;
                    for &(w, c) in &g[v] {
                        if par == w || vis[w] {
                            continue;
                        }
                        if dp[w] > sz / 2 {
                            dist += c;
                            par = v;
                            v = w;
                            has_majority = true;
                            break;
                        }
                    }
                    if !has_majority {
                        return (v, dist);
                    }
                }
            };
            find_centroid(v, n)
        };
        let g_cent = g[cent].clone();
        if par < n {
            edges.push((par, cent, dist + cost));
        }
        // v was selected as a centroid
        // and will be ignored in the following decomposition procedure
        vis[cent] = true;
        for &(w, c) in &g_cent {
            if !vis[w] {
                centroid_decompose_inner(g, w, cent, c, edges, dp, vis);
            }
        }
    }
    let n = g.len();
    let mut edges = vec![];
    // This Vec is reused many times
    let mut dp = vec![0; n];
    let mut vis = vec![false; n];
    centroid_decompose_inner(&g, 0, n, 0, &mut edges, &mut dp, &mut vis);
    edges
}

fn dfs1(
    v: usize,
    par: usize,
    g: &[Vec<(usize, i64)>],
    used: &mut [bool],
    dists: &mut Vec<(usize, i64)>,
    now: i64,
) {
    if used[v] {
        return;
    }
    dists.push((v, now));
    for &(w, c) in &g[v] {
        if w == par {
            continue;
        }
        dfs1(w, v, g, used, dists, now + c);
    }
}

fn self_merge(dist: &[(usize, i64)], x: i64) -> (i64, i64) {
    let n = dist.len();
    let mut ds: Vec<i64> = dist.iter().map(|&a| a.1).collect();
    ds.sort();
    let mut ltx = 0;
    let mut gex = 0;
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + ds[i];
    }
    for &d in &ds {
        let idx = ds.lower_bound(&(x - d));
        ltx += idx as i64 * d + acc[idx];
        gex += (n - idx) as i64;
    }
    (ltx, gex)
}

fn dfs2(
    v: usize,
    x: i64,
    g: &[Vec<(usize, i64)>],
    chcd: &[Vec<usize>],
    used: &mut [bool],
) -> (i64, i64) {
    let mut all = vec![];
    let mut ltx = 0;
    let mut gex = 0;
    for &(w, c) in &g[v] {
        let mut dists = vec![];
        dfs1(w, v, g, used, &mut dists, c);
        let (a, b) = self_merge(&dists, x);
        ltx -= a; 
        gex -= b;
        all.extend_from_slice(&dists);
    }
    all.push((v, 0));
    let (a, b) = self_merge(&all, x);
    ltx += a;
    gex += b;
    used[v] = true;
    for &w in &chcd[v] {
        let sub = dfs2(w, x, g, chcd, used);
        ltx += sub.0;
        gex += sub.1;
    }
    (ltx, gex)
}

// The author read the solution before implementing this.
// Tags: centroid-decomposition
fn solve() {
    input! {
        n: usize, x: i64,
        abc: [(usize1, usize1, i64); n - 1],
    }
    let mut g = vec![vec![]; n];
    let mut miw = vec![1i64 << 50; n];
    for &(a, b, c) in &abc {
        g[a].push((b, c));
        g[b].push((a, c));
        miw[a].chmin(c);
        miw[b].chmin(c);
    }
    let cd = centroid_decompose(&g);
    let mut chcd = vec![vec![]; n];
    for &(a, b, _) in &cd {
        chcd[a].push(b);
    }
    let mut used = vec![false; n];
    let (ltx, gex) = dfs2(cd[0].0, x, &g, &chcd, &mut used);
    eprintln!("ltx = {}, gex = {}", ltx, gex);
    let mut tot = (ltx + gex * x) / 2;
    for &(a, b, c) in &abc {
        let mut mi = c;
        if g[a].len() + g[b].len() != n {
            mi.chmin(2 * x); // TODO
        }
        mi.chmin(x + min(miw[a], miw[b]));
        tot += mi - min(c, x);
    }
    println!("{}", tot);
}
